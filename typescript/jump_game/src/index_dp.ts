// https://leetcode.com/problems/jump-game

export function jump_game(nums: number[]): boolean {
  const dp = []

  dp[nums.length - 1] = true

  for (let i = 0; i < nums.length - 1; i++) {
    dp[i] = false
  }

  for (let i = nums.length - 2; i >= 0; i--) {
    for (let j = 1; j <= nums[i]; j++) {
      if (dp[i+j] === true) {
        dp[i] = true
        break
      }
    }
  }

  return dp[0]
}
