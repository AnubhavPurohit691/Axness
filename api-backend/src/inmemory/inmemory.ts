type User = {
  Id: string,
  username: string,
  password: string,
  balance: {
    coins: Record<string, number>,
    usd: number
  }
}
export type Trade = {
  stopLoss?: number | null,
  status: "open" | "closed",
  tradeId: string,
  takeProfit?: number | null,
  pnl?: number,
  openPrice: number,
  leverage: number,
  quantity: number,
  userId: string,
  asset: string,
  closedPrice?: number,
  marginPrice: number
}
export const trades: Trade[] = []
export const liquidity = new Map<string, Array<Trade>>()
export const users: User[] = []