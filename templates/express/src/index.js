import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import messageRouter from "./routes/message.js";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 8000;

app.use(cors());
app.use(express.json());

// API routes
app.use("/api/message", messageRouter);

app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
