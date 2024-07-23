// https://leetcode.com/problems/best-time-to-buy-and-sell-stock

export function best_time_to_buy_and_sell_stock(prices: number[]): number {
  const r = [0]
  const mins = [prices[0]]

  for (let i = 1; i < prices.length; i++) {
    const ri = Math.max(r[i - 1], prices[i] - mins[i-1])
    r[i] = ri
    mins[i] = Math.min(prices[i], mins[i-1])
  }
  return r[r.length - 1]
}
