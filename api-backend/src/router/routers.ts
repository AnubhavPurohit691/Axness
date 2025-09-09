import {Router} from "express"
import { signincontroller, signupcontroller } from "../controller/usercontroller";
import { authmiddleware } from "../middleware/middleware";
import { closedtrades, dotrading } from "../controller/order/order";

const router = Router()
router.post("/signup",signupcontroller)
router.post("/signin",signincontroller)
// router.get("/getorder", authmiddleware, getordercontroller);
// router.get("/balance", authmiddleware, getbalance);
// router.get("/candles", getcandlesController)
router.post("/opentrades",  dotrading)
// router.get("/trades/open", authmiddleware, gettrades)
router.post("/closetrades", authmiddleware, closedtrades)
export default router;