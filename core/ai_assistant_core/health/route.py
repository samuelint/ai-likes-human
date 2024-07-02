from fastapi import FastAPI


def bind_health_routes(app: FastAPI):

    @app.get("/health")
    @app.get("/")
    async def health():
        return {"status": "ok"}
