// https://leetcode.com/problems/construct-k-palindrome-strings/description/

export function construct_k_palindrome_strings(s: string, k: number): boolean {
  if (s.length < k) return false;
  const counts: Map<string, number> = new Map();

  for (const char of s) {
    const curr = counts.get(char) || 0;
    counts.set(char, curr + 1);
  }

  let oddCount = 0;
  counts.forEach((v) => {
    if (v % 2 === 1) oddCount += 1;
  })

  if (oddCount > k) return false

  return true
}
