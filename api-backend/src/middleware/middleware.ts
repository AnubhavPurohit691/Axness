import type { NextFunction, Request, Response } from "express"
import jwt, { type JwtPayload } from "jsonwebtoken"
export interface AuthenticatedRequest extends Request {
  userId?: string;
}

export const authmiddleware = (req: AuthenticatedRequest, res: Response, next: NextFunction) => {
  const token = req.cookies.gettoken;
  console.log(token)
  if (!token) {
    return res.status(401).json({ message: "Unauthorized" });
  }

  const decoded = jwt.verify(token, process.env.Secret || "anubhav") as JwtPayload
  req.userId = decoded.userId;
  next();
}