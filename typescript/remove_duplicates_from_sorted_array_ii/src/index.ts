// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii

export function remove_duplicates_from_sorted_array_II(nums: number[]): number {
  let idx = 0 // Index to write to
  let ib = 0 // Beginning of repeated range
  let ie = 0 // End of repeated range
  let k = 0

  while (ib < nums.length) {
    let currChar = nums[ib]
    while (ie < nums.length && nums[ie] === nums[ib]) {
      ie++
    }
    if (ie - ib === 1) {
      nums[idx++] = currChar
      k += 1
    } else {
      nums[idx++] = currChar
      nums[idx++] = currChar
      k += 2
    }
    ib = ie
  }

  return k
}
