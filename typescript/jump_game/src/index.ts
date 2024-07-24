// https://leetcode.com/problems/jump-game

export function jump_game(nums: number[]): boolean {
  const memo: Set<number> = new Set()

  function jump_game_(end: number): boolean {
    if (end == 0) {
      return true
    }

    if (memo.has(end)) {
      return false
    }

    for (let i = 0; i <= end - 1; i++) {
      if (nums[i] >= end - i) {
        if (jump_game_(i)) {
          return true
        }
      }
    }

    memo.add(end)

    return false
  }

  return jump_game_(nums.length - 1)
}
