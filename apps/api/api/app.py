"""
This is the main module of the API service.
"""

import importlib.metadata

import importlib

import fastapi
import pydantic
import httpx

class EvalRequest(pydantic.BaseModel):
    code: str

class AbanosAPI(fastapi.FastAPI):
    """
    Abanos API server.

    Attributes:
        backend_uri (str): Backend URI

    """
    backend_uri: str

    def __init__(self, backend_uri: str) -> None:
        super().__init__(
            title="Abanos API",
            description="API for the Abanos platform",
            version=self.get_version(),
        )

        self.backend_uri = backend_uri
        
        self.client = httpx.AsyncClient()
        self.add_event_handler("shutdown", self.close_http_client)
        
        self.add_api_route("/healthz", self.healthz, methods=["GET"], status_code=200)
        self.add_api_route("/eval", self.eval, methods=["POST"])

    @staticmethod
    def get_version() -> str:
        try:
            return importlib.metadata.version("abanos-api")
        except importlib.metadata.PackageNotFoundError:
            return "0.0.0-dev"
            
    async def healthz(self) -> dict:
        return {"status": "ok"}

    async def eval(self, request: EvalRequest) -> dict:
        """
        Evaluates the given code.
        """
        try:
            r = await self.client.post(
                f"{self.backend_uri}/eval",
                json=request.model_dump_json(),
            )
        except httpx.ConnectError as e:
            raise fastapi.HTTPException(status_code=503, detail="Backend service is unavailable") from e

        r.raise_for_status()
        return r.json()
    
    async def close_http_client(self) -> None:
        await self.client.aclose()
