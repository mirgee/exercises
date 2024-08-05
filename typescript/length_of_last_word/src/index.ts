export function length_of_last_word(s: string): number {
  return s.trimEnd().split(" ").pop()?.length!
}
