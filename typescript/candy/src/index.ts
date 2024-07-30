export function candy(ratings: number[]): number {
  const candies = []
  for (let i = 0; i < ratings.length; i++) {
    candies.push(1)
  }

  for (let i = 1; i < ratings.length; i++) {
    if (ratings[i] > ratings[i - 1]) {
      candies[i] = candies[i - 1] + 1
    }
  }

  for (let i = ratings.length - 1; i >= 0; i--) {
    if (ratings[i + 1] < ratings[i] && candies[i+1] >= candies[i]) {
      candies[i] = candies[i + 1] + 1
    }
  }

  return candies.reduce((previous, current) => previous + current, 0)
}
