// TODO(M1): Orchestrate parse -> UID generation -> store.
//
// The indexer will:
// 1. Walk project files
// 2. Compute content hashes (skip unchanged files)
// 3. Parse via LanguageAdapter
// 4. Generate UIDs and fingerprints
// 5. Upsert into Repository
