// https://leetcode.com/problems/rotate-array

export function rotate_array(nums: number[], k: number) {
  nums.reverse()
  let delim = k % nums.length

  let l = 0
  let r = delim - 1

  while (l < r) {
    const t = nums[l]
    nums[l] = nums[r]
    nums[r] = t
    l++
    r--
  }

  l = delim
  r = nums.length - 1

  while (l < r) {
    const t = nums[l]
    nums[l] = nums[r]
    nums[r] = t
    l++
    r--
  }

  return nums
}
