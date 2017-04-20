// @flow

export function sorted<T>(arr: T[], compareFn: (T, T) => number): T[] {
  return arr.slice(0).sort(compareFn);
}
