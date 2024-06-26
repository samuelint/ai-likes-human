from fastapi import HTTPException


def get_bearer_token(auth_header: str) -> str:
    if auth_header and auth_header.startswith("Bearer "):
        return auth_header[len("Bearer "):]
    raise HTTPException(status_code=401, detail="Invalid authorization header")
