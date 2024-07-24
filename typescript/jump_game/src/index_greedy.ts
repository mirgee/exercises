// https://leetcode.com/problems/jump-game

export function jump_game(nums: number[]): boolean {
  let i = 0
  let gas_remaining = 0

  while (i < nums.length) {
    if (nums[i] > gas_remaining) {
      gas_remaining = nums[i]
    }
    if (gas_remaining === 0 && i < nums.length - 1) return false
    gas_remaining--
    i++
  }

  return true
}
