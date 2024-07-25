// https://leetcode.com/problems/jump-game-ii

export function jump_game_ii(nums: number[]): number {
   const dp = []

  dp[nums.length - 1] = 1

  for (let i = 0; i < nums.length - 1; i++) {
    dp[i] = 0
  }

  for (let i = nums.length - 2; i >= 0; i--) {
    let min = Number.MAX_VALUE
    for (let j = 1; j <= nums[i]; j++) {
      if (dp[i+j] > 0 && dp[i+j] < min) {
        min = dp[i+j] 
      }
    }
    dp[i] = min + 1
  }

  return dp[0] - 1
}
