export function gas_station(gas: number[], cost: number[]): number {
  let total_sum = 0
  let solution = 0
  let current_sum = 0

  for (let i = 0; i < gas.length; i++) {
    total_sum += gas[i] - cost[i]
    current_sum += gas[i] - cost[i]

    if (current_sum < 0) {
      current_sum = 0
      solution = i + 1
    }
  }

  if (total_sum < 0) return -1

  return solution
}
