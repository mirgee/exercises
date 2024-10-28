// https://leetcode.com/problems/longest-palindromic-substring

export function longest_palindromic_substring(s: string): string {
  let longest = '';

  function expandAroundCenter(left: number, right: number): string {
    while (left >= 0 && right < s.length && s[left] === s[right]) {
      left -= 1;
      right += 1;
    }
    return s.substring(left + 1, right);
  }

  for (let i = 0; i < s.length; i++) {
    const odd = expandAroundCenter(i, i);
    const even = expandAroundCenter(i, i + 1);

    const longer = odd.length > even.length ? odd : even;
    longest = longer.length > longest.length ? longer : longest;
  }

  return longest;
}
