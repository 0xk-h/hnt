import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import messageRouter from "./routes/message.ts";

dotenv.config();

const app: Application = express();
const PORT: string | number = process.env.PORT || 5000;

app.use(cors());
app.use(express.json());

// API routes
app.use("/api/message", messageRouter);

app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
