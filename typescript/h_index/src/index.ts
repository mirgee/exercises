export function h_index(citations: number[]): number {
  citations.sort((a, b) => b - a)
  let i = 0
  while (citations[i] >= i + 1) {
    i++
  }
  return i
}

export function h_index_0(citations: number[]): number {
  citations.sort((a, b) => b - a)
  for (let i = 0; i < citations.length; i++) {
    if (citations[i] < i + 1) return i
  }
  return 1
}
