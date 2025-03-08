"""main.py

Launches the Abanos API server.

Usage:
    python main.py --backend-uri http://localhost:8001

Options:
    --host: Host to bind to
    --port: Port to bind to
    --backend-uri: Backend URI
    --reload: Enable auto-reload
"""

import click
import uvicorn

import app

@click.command()
@click.option("--host", default="0.0.0.0", type=str, help="Host to bind to")
@click.option("--port", default=8000, type=int, help="Port to bind to")
@click.option("--backend-uri", help="Backend URI", required=True)
def main(host: str, port: int, backend_uri: str) -> None:
    """
    Launches the Abanos API server.

    Args:
        host: Host to bind to
        port: Port to bind to
        backend_uri: Backend URI
        reload: Enable auto-reload

    Returns:
        None
    """ 
    
    uvicorn.run(
        app=app.AbanosAPI(backend_uri),
        host=host,
        port=port,
    )


if __name__ == "__main__":
    main(auto_envvar_prefix="ABANOS_API")
