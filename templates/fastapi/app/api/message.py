from fastapi import APIRouter

router = APIRouter()

@router.get("/message")
def get_message():
    return { "message": "All set! Time to build something awesome" }
