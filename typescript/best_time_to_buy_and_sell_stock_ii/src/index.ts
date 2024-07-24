// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

export function best_time_to_buy_and_sell_stock_ii(prices: number[]): number {
  let profit = 0
  let l = 0
  let r = 1

  while (r < prices.length) {
    if (prices[l] >= prices[r]) {
      l = r
    } else {
      profit += prices[r] - prices[l++]
    }
    r++
  }

  return profit
}
