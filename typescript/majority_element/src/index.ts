// https://leetcode.com/problems/majority-element

export function majority_element(nums: number[]): number {
  let candidate = Number.MIN_VALUE
  let count = 0

  for (let num of nums) {
    if (num === candidate) {
      count++
    } else if (count > 0) {
      count--
    } else {
      candidate = num
      count++
    }
  }

  return candidate
}
