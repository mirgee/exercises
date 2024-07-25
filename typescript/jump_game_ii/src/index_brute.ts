// https://leetcode.com/problems/jump-game-ii

export function jump_game_ii(nums: number[], pos = 0): number {
  if (pos >= nums.length - 1) return 0

  let min = Number.MAX_VALUE
  for (let i = 1; i <= nums[pos]; i++) {
    min = Math.min(min, 1 + jump_game_ii(nums, pos + i))
  }

  return min
}
