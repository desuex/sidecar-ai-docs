use std::cell::RefCell;

use tree_sitter::{Node, Parser};

use crate::adapter::{LanguageAdapter, RawRef, RawSymbol};
use sidecar_types::{Language, Range, RefKind, SymbolKind, Visibility};

/// TypeScript/JavaScript language adapter using Tree-sitter.
pub struct TypeScriptAdapter {
    /// RefCell because tree-sitter Parser::parse requires &mut self,
    /// but our LanguageAdapter trait uses &self.
    parser: RefCell<Parser>,
}

// Safety: TypeScriptAdapter is not meant to be shared across threads.
// Each thread should own its own adapter. The RefCell is only for
// interior mutability within single-threaded usage.
unsafe impl Send for TypeScriptAdapter {}
unsafe impl Sync for TypeScriptAdapter {}

impl TypeScriptAdapter {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("Error loading TypeScript parser");
        TypeScriptAdapter {
            parser: RefCell::new(parser),
        }
    }
}

impl Default for TypeScriptAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for TypeScriptAdapter {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn parse_symbols(&self, source: &[u8]) -> Vec<RawSymbol> {
        let tree = match self.parser.borrow_mut().parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut symbols = Vec::new();
        let root = tree.root_node();
        collect_symbols(source, &root, None, false, &mut symbols);

        // Sort by range.start for deterministic output
        symbols.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        symbols
    }

    fn parse_refs(&self, source: &[u8]) -> Vec<RawRef> {
        let tree = match self.parser.borrow_mut().parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut refs = Vec::new();
        let root = tree.root_node();
        collect_refs(source, &root, None, &mut refs);

        refs.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        refs
    }
}

/// Recursively walk the AST and collect symbol definitions.
fn collect_symbols(
    source: &[u8],
    node: &Node,
    parent_name: Option<&str>,
    is_exported: bool,
    out: &mut Vec<RawSymbol>,
) {
    let kind_str = node.kind();

    match kind_str {
        "export_statement" => {
            // Children of export_statement inherit export visibility
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    collect_symbols(source, &child, parent_name, true, out);
                }
            }
            return;
        }
        "class_declaration" => {
            if let Some(sym) = extract_class(source, node, is_exported) {
                let class_name = sym.name.clone();
                out.push(sym);
                // Collect methods inside the class body
                if let Some(body) = node.child_by_field_name("body") {
                    for i in 0..body.child_count() {
                        if let Some(child) = body.child(i) {
                            collect_symbols(source, &child, Some(&class_name), is_exported, out);
                        }
                    }
                }
                return;
            }
        }
        "method_definition" | "public_field_definition" => {
            if let Some(sym) = extract_method(source, node, parent_name, is_exported) {
                out.push(sym);
                return;
            }
        }
        "function_declaration" => {
            if let Some(sym) = extract_function(source, node, is_exported) {
                out.push(sym);
                return;
            }
        }
        "interface_declaration" => {
            if let Some(sym) = extract_named_decl(source, node, SymbolKind::Interface, is_exported)
            {
                out.push(sym);
                return;
            }
        }
        "enum_declaration" => {
            if let Some(sym) = extract_named_decl(source, node, SymbolKind::Enum, is_exported) {
                out.push(sym);
                return;
            }
        }
        "type_alias_declaration" => {
            if let Some(sym) = extract_named_decl(source, node, SymbolKind::Type, is_exported) {
                out.push(sym);
                return;
            }
        }
        "lexical_declaration" | "variable_declaration" => {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "variable_declarator" {
                        if let Some(sym) = extract_variable(source, &child, kind_str, is_exported) {
                            out.push(sym);
                        }
                    }
                }
            }
            return;
        }
        _ => {}
    }

    // Recurse into children for unmatched nodes
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_symbols(source, &child, parent_name, is_exported, out);
        }
    }
}

fn node_text<'a>(source: &'a [u8], node: &Node) -> &'a str {
    std::str::from_utf8(&source[node.start_byte()..node.end_byte()]).unwrap_or("")
}

fn extract_class(source: &[u8], node: &Node, is_exported: bool) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();
    let fingerprint_input = format!("class_declaration\nname: {name}");
    Some(RawSymbol {
        qualified_name: name.clone(),
        name,
        kind: SymbolKind::Class,
        visibility: if is_exported {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

fn extract_method(
    source: &[u8],
    node: &Node,
    parent_name: Option<&str>,
    _is_exported: bool,
) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();
    let parent = parent_name.unwrap_or("");
    let qualified_name = if parent.is_empty() {
        name.clone()
    } else {
        format!("{parent}.{name}")
    };

    let params = extract_params_text(source, node);
    let fingerprint_input =
        format!("method_definition\nname: {name}\nparent: {parent}\nparameters: [{params}]");

    let visibility = if has_accessibility_modifier(source, node, "private") {
        Visibility::Private
    } else if has_accessibility_modifier(source, node, "protected") {
        Visibility::Protected
    } else {
        Visibility::Public // methods are public by default in TS
    };

    Some(RawSymbol {
        qualified_name,
        name,
        kind: SymbolKind::Method,
        visibility,
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

fn extract_function(source: &[u8], node: &Node, is_exported: bool) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();
    let params = extract_params_text(source, node);
    let fingerprint_input = format!("function_declaration\nname: {name}\nparameters: [{params}]");

    Some(RawSymbol {
        qualified_name: name.clone(),
        name,
        kind: SymbolKind::Function,
        visibility: if is_exported {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

fn extract_named_decl(
    source: &[u8],
    node: &Node,
    kind: SymbolKind,
    is_exported: bool,
) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();
    let kind_str = node.kind();
    let fingerprint_input = format!("{kind_str}\nname: {name}");

    Some(RawSymbol {
        qualified_name: name.clone(),
        name,
        kind,
        visibility: if is_exported {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

fn extract_variable(
    source: &[u8],
    node: &Node,
    decl_kind: &str,
    is_exported: bool,
) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();

    let sym_kind = if decl_kind == "lexical_declaration" {
        SymbolKind::Constant
    } else {
        SymbolKind::Variable
    };

    let fingerprint_input = format!("variable_declarator\nname: {name}\ndecl: {decl_kind}");

    Some(RawSymbol {
        qualified_name: name.clone(),
        name,
        kind: sym_kind,
        visibility: if is_exported {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

/// Extract parameter text for fingerprint (normalized whitespace).
fn extract_params_text(source: &[u8], node: &Node) -> String {
    if let Some(params) = node.child_by_field_name("parameters") {
        let text = node_text(source, &params);
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    }
}

/// Check if a node has an accessibility_modifier child matching the given keyword.
fn has_accessibility_modifier(source: &[u8], node: &Node, modifier: &str) -> bool {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "accessibility_modifier" && node_text(source, &child) == modifier {
                return true;
            }
        }
    }
    false
}

/// Recursively walk the AST and collect references.
fn collect_refs(source: &[u8], node: &Node, enclosing: Option<&str>, out: &mut Vec<RawRef>) {
    let kind_str = node.kind();

    match kind_str {
        // import { Foo, Bar } from "./module"
        "import_statement" => {
            if let Some(clause) = node.child_by_field_name("source") {
                // Find import specifiers
                for i in 0..node.child_count() {
                    if let Some(child) = node.child(i) {
                        if child.kind() == "import_clause" || child.kind() == "named_imports" {
                            collect_import_names(source, &child, node, out);
                        }
                        // Recurse into import_clause to find named_imports
                        if child.kind() == "import_clause" {
                            for j in 0..child.child_count() {
                                if let Some(gc) = child.child(j) {
                                    if gc.kind() == "named_imports" {
                                        collect_import_names(source, &gc, node, out);
                                    }
                                }
                            }
                        }
                    }
                }
                let _ = clause; // used above for context
            }
            return;
        }
        // function call: foo(), this.bar(), obj.method()
        "call_expression" => {
            if let Some(func) = node.child_by_field_name("function") {
                let call_name = extract_call_name(source, &func);
                if !call_name.is_empty() && !is_builtin_call(&call_name) {
                    let from = enclosing.unwrap_or("<file>").to_owned();
                    out.push(RawRef {
                        from_qualified_name: from,
                        to_name: call_name,
                        range: Range {
                            start: node.start_byte() as u32,
                            end: node.end_byte() as u32,
                        },
                        ref_kind: RefKind::Call,
                    });
                }
            }
        }
        // Type annotations: x: Foo, param: Bar[]
        "type_identifier" => {
            let name = node_text(source, node).to_owned();
            if !is_builtin_type(&name) {
                let from = enclosing.unwrap_or("<file>").to_owned();
                out.push(RawRef {
                    from_qualified_name: from,
                    to_name: name,
                    range: Range {
                        start: node.start_byte() as u32,
                        end: node.end_byte() as u32,
                    },
                    ref_kind: RefKind::TypeRef,
                });
            }
            return; // no children
        }
        // extends/implements
        "extends_clause" => {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" || child.kind() == "type_identifier" {
                        let name = node_text(source, &child).to_owned();
                        let from = enclosing.unwrap_or("<file>").to_owned();
                        out.push(RawRef {
                            from_qualified_name: from,
                            to_name: name,
                            range: Range {
                                start: child.start_byte() as u32,
                                end: child.end_byte() as u32,
                            },
                            ref_kind: RefKind::Inherit,
                        });
                    }
                }
            }
            return;
        }
        // Track enclosing scope
        "class_declaration" => {
            let class_name = node
                .child_by_field_name("name")
                .map(|n| node_text(source, &n).to_owned());
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    collect_refs(source, &child, class_name.as_deref().or(enclosing), out);
                }
            }
            return;
        }
        "method_definition" => {
            let method_name = node.child_by_field_name("name").map(|n| {
                let name = node_text(source, &n);
                match enclosing {
                    Some(parent) => format!("{parent}.{name}"),
                    None => name.to_owned(),
                }
            });
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    collect_refs(source, &child, method_name.as_deref().or(enclosing), out);
                }
            }
            return;
        }
        "function_declaration" => {
            let fn_name = node
                .child_by_field_name("name")
                .map(|n| node_text(source, &n).to_owned());
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    collect_refs(source, &child, fn_name.as_deref().or(enclosing), out);
                }
            }
            return;
        }
        _ => {}
    }

    // Default: recurse
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_refs(source, &child, enclosing, out);
        }
    }
}

fn collect_import_names(source: &[u8], node: &Node, import_node: &Node, out: &mut Vec<RawRef>) {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "import_specifier" || child.kind() == "identifier" {
                let name_node = child.child_by_field_name("name").unwrap_or(child);
                let name = node_text(source, &name_node).to_owned();
                if !name.is_empty() && name != "{" && name != "}" && name != "," {
                    out.push(RawRef {
                        from_qualified_name: "<file>".to_owned(),
                        to_name: name,
                        range: Range {
                            start: import_node.start_byte() as u32,
                            end: import_node.end_byte() as u32,
                        },
                        ref_kind: RefKind::Import,
                    });
                }
            }
        }
    }
}

fn extract_call_name(source: &[u8], func_node: &Node) -> String {
    match func_node.kind() {
        "identifier" => node_text(source, func_node).to_owned(),
        "member_expression" => {
            // obj.method or this.method — extract the method name
            if let Some(prop) = func_node.child_by_field_name("property") {
                node_text(source, &prop).to_owned()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

fn is_builtin_call(name: &str) -> bool {
    matches!(
        name,
        "push"
            | "pop"
            | "splice"
            | "reduce"
            | "map"
            | "filter"
            | "forEach"
            | "find"
            | "length"
            | "toString"
            | "valueOf"
            | "console"
            | "log"
            | "error"
            | "warn"
            | "parseInt"
            | "parseFloat"
            | "require"
    )
}

fn is_builtin_type(name: &str) -> bool {
    matches!(
        name,
        "string"
            | "number"
            | "boolean"
            | "void"
            | "null"
            | "undefined"
            | "any"
            | "never"
            | "unknown"
            | "object"
            | "symbol"
            | "bigint"
            | "String"
            | "Number"
            | "Boolean"
            | "Object"
            | "Array"
            | "Promise"
            | "Map"
            | "Set"
            | "Date"
            | "RegExp"
            | "Error"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use sidecar_types::RefKind;

    const SAMPLE_TS: &[u8] = br#"
export class CartService {
  private items: string[] = [];

  addItem(item: string): void {
    this.items.push(item);
  }

  calculateTotal(): number {
    return this.items.length;
  }
}

export function createCart(): CartService {
  return new CartService();
}

const TAX_RATE = 0.08;

interface Config {
  debug: boolean;
}

export enum Status {
  Active,
  Inactive,
}

export type ID = string;
"#;

    #[test]
    fn extracts_symbols_from_typescript() {
        let adapter = TypeScriptAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_TS);

        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"CartService"), "missing class: {names:?}");
        assert!(names.contains(&"addItem"), "missing method: {names:?}");
        assert!(
            names.contains(&"calculateTotal"),
            "missing method: {names:?}"
        );
        assert!(names.contains(&"createCart"), "missing function: {names:?}");
        assert!(names.contains(&"TAX_RATE"), "missing const: {names:?}");
        assert!(names.contains(&"Config"), "missing interface: {names:?}");
        assert!(names.contains(&"Status"), "missing enum: {names:?}");
        assert!(names.contains(&"ID"), "missing type alias: {names:?}");
    }

    #[test]
    fn qualified_names_for_methods() {
        let adapter = TypeScriptAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_TS);

        let add_item = symbols
            .iter()
            .find(|s| s.name == "addItem")
            .expect("addItem not found");
        assert_eq!(add_item.qualified_name, "CartService.addItem");
        assert_eq!(add_item.kind, SymbolKind::Method);
    }

    #[test]
    fn export_visibility() {
        let adapter = TypeScriptAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_TS);

        let cart = symbols.iter().find(|s| s.name == "CartService").unwrap();
        assert_eq!(cart.visibility, Visibility::Public);

        let tax = symbols.iter().find(|s| s.name == "TAX_RATE").unwrap();
        assert_eq!(tax.visibility, Visibility::Private);

        let config = symbols.iter().find(|s| s.name == "Config").unwrap();
        assert_eq!(config.visibility, Visibility::Private);
    }

    #[test]
    fn deterministic_output() {
        let adapter = TypeScriptAdapter::new();
        let s1 = adapter.parse_symbols(SAMPLE_TS);
        let s2 = adapter.parse_symbols(SAMPLE_TS);

        let names1: Vec<&str> = s1.iter().map(|s| s.name.as_str()).collect();
        let names2: Vec<&str> = s2.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names1, names2);
    }

    #[test]
    fn sorted_by_range() {
        let adapter = TypeScriptAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_TS);

        for window in symbols.windows(2) {
            assert!(
                window[0].range.start <= window[1].range.start,
                "symbols not sorted by range"
            );
        }
    }

    const REFS_TS: &[u8] = br#"
import { CartItem, Currency } from "./types";
import { formatCurrency } from "./utils";

export class CartService {
  private items: CartItem[] = [];

  addItem(item: CartItem): void {
    this.items.push(item);
  }

  calculateTotal(currency: Currency): number {
    return 42;
  }

  formatTotal(currency: Currency): string {
    const total = this.calculateTotal(currency);
    return formatCurrency(total, currency);
  }
}
"#;

    #[test]
    fn extracts_import_refs() {
        let adapter = TypeScriptAdapter::new();
        let refs = adapter.parse_refs(REFS_TS);
        let imports: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::Import)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            imports.contains(&"CartItem"),
            "missing CartItem import: {imports:?}"
        );
        assert!(
            imports.contains(&"Currency"),
            "missing Currency import: {imports:?}"
        );
        assert!(
            imports.contains(&"formatCurrency"),
            "missing formatCurrency import: {imports:?}"
        );
    }

    #[test]
    fn extracts_call_refs() {
        let adapter = TypeScriptAdapter::new();
        let refs = adapter.parse_refs(REFS_TS);
        let calls: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::Call)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            calls.contains(&"calculateTotal"),
            "missing calculateTotal call: {calls:?}"
        );
        assert!(
            calls.contains(&"formatCurrency"),
            "missing formatCurrency call: {calls:?}"
        );
    }

    #[test]
    fn extracts_type_refs() {
        let adapter = TypeScriptAdapter::new();
        let refs = adapter.parse_refs(REFS_TS);
        let types: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::TypeRef)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            types.contains(&"CartItem"),
            "missing CartItem type ref: {types:?}"
        );
        assert!(
            types.contains(&"Currency"),
            "missing Currency type ref: {types:?}"
        );
    }
}
