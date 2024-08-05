export function rle_modified(s: string) {
  if (s.length === 1) {
    return s
  }
  let count = 1
  let res = ""
  for (let i = 1; i <= s.length; i++) {
    if (s[i] === s[i - 1]) {
      count += 1
    } else {
      if (count === 1) {
        res += s[i - 1]
      } else if (count === 2) {
        res += `${s[i - 1]}${s[i - 1]}`
      } else {
        res += `${count}${s[i - 1]}`
      }
      count = 1
    }
  }

  return res
}
