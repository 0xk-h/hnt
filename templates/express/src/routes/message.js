import express from "express";

const router = express.Router();

router.get("/", (req, res) => {
  res.json({ message: "All set! Time to build something awesome" });
});

export default router;
