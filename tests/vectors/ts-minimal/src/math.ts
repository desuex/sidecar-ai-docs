export type NumberPair = {
  left: number;
  right: number;
};

export function add(pair: NumberPair): number {
  return pair.left + pair.right;
}
