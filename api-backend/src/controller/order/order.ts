import type { Response } from "express";
import { producer } from "../..";
import type { AuthenticatedRequest } from "../../middleware/middleware";

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
export async function dotrading(req: AuthenticatedRequest, res: Response) {
    const data:Trade = req.body
    await producer.send({
        topic: "yoyo",
        messages: [{
            value: JSON.stringify(
                data
            )
        }]
    })
}