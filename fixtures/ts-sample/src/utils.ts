export function formatCurrency(amount: number, currency: string): string {
  return `${currency} ${amount.toFixed(2)}`;
}

export function roundTo(value: number, decimals: number): number {
  const factor = Math.pow(10, decimals);
  return Math.round(value * factor) / factor;
}

const TAX_RATE = 0.08;

export function withTax(amount: number): number {
  return roundTo(amount * (1 + TAX_RATE), 2);
}
