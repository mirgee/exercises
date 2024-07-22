// https://leetcode.com/problems/remove-element

export function remove_element_0(nums: number[], val: number): number {
  let k = 0
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] === val) {
      nums[i] = -1
      k++
    }
  }
  nums.sort().reverse()
  return nums.length - k
}

export function remove_element(nums: number[], val: number): number {
  let idx = 0
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] !== val) {
      nums[idx++] = nums[i]
    }
  }
  return idx
}
