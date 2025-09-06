import type { Request, Response } from "express"
import { randomUUID } from "crypto"
import jwt from "jsonwebtoken"
// import "dotenv/config"
// import { users } from "../inmemory/inmemory"

export const signupcontroller = (req: Request, res: Response) => {
  const data = req.body
  console.log(data)
  if (data.username === null && data.password === null) return
    const userId = randomUUID()
  const token = jwt.sign({ userId: userId }, process.env.Secret || "anubhav")
  res.cookie("gettoken", token)
  return res.json({ message: "cookie set", token })
}
export const signincontroller = (req: Request, res: Response) => {
  const username = req.body.username;
  const password = req.body.password;
  const userId = username
  const token = jwt.sign({ userId: userId }, process.env.Secret || "anubhav")
  res.cookie("gettoken", token)
  return res.json({ username })
}