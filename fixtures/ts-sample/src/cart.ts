import { CartItem, Currency } from "./types";
import { formatCurrency, roundTo } from "./utils";

export class CartService {
  private items: CartItem[] = [];

  addItem(item: CartItem): void {
    this.items.push(item);
  }

  removeItem(index: number): void {
    this.items.splice(index, 1);
  }

  calculateTotal(currency: Currency): number {
    const sum = this.items.reduce((acc, item) => acc + item.price * item.quantity, 0);
    return roundTo(sum, 2);
  }

  formatTotal(currency: Currency): string {
    const total = this.calculateTotal(currency);
    return formatCurrency(total, currency);
  }
}

export function createCart(): CartService {
  return new CartService();
}
