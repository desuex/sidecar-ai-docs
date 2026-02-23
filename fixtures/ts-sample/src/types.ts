export interface CartItem {
  name: string;
  price: number;
  quantity: number;
}

export enum Currency {
  USD = "USD",
  EUR = "EUR",
  GBP = "GBP",
}

export type Price = number;

export type CartSummary = {
  items: CartItem[];
  total: Price;
  currency: Currency;
};
