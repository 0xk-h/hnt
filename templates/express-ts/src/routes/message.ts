import Router from "express";

const router = Router();

router.get("/", (req: Request, res: Response) => {
  res.json({ message: "All set! Time to build something awesome" });
});

export default router;
