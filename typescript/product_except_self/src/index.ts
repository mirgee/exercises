export function product_except_self(nums: number[]): number[] {
  const prefix = []
  const suffix = []
  const res = []

  prefix[0] = 1
  suffix[nums.length - 1] = 1

  for (let i = 1; i < nums.length; i++) {
    prefix[i] = prefix[i - 1] * nums[i - 1]
  }

  for (let i = nums.length - 2; i >= 0; i--) {
    suffix[i] = suffix[i + 1] * nums[i + 1]
  }

  for (let i = 0; i < nums.length; i++) {
    res[i] = prefix[i] * suffix[i]
  }

  return res
}
