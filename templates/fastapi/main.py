from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.api import message

app = FastAPI(title="FastAPI Scaffold Template")

# Allow your frontend origin (or "*" for all)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173"],  # replace with your frontend URL
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(message.router, prefix="/api")
