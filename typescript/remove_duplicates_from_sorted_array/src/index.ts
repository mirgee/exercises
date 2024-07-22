// https://leetcode.com/problems/remove-duplicates-from-sorted-array

export function remove_duplicates_from_sorted_array(nums: number[]): number {
  let prev = -1000
  let idx = 0
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] === prev) {
      continue
    }
    nums[idx++] = nums[i]
    prev = nums[i]
  }
  return idx
}
