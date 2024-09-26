export function maxProfit(prices: number[]): number {
  let minPrice = prices[0];
  let maxProfit = -Infinity;
  const length = prices.length;
  for (let i = 1; i < length; i++) {
    const profit = prices[i] - minPrice;
    if (profit > maxProfit) {
      maxProfit = profit;
    }
    if (prices[i] < minPrice) {
      minPrice = prices[i];
    }
  }

  return maxProfit < 0 ? 0 : maxProfit;
}
