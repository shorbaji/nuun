import pytest
from unittest.mock import patch, AsyncMock
import importlib.metadata
import httpx

from api.app import AbanosAPI

def test_get_version():
    with patch("importlib.metadata.version", return_value="1.0.0"):
        assert AbanosAPI.get_version() == "1.0.0"
    
    with patch("importlib.metadata.version", side_effect=importlib.metadata.PackageNotFoundError):
        assert AbanosAPI.get_version() == "0.0.0-dev"

@pytest.fixture
def api():
    return AbanosAPI("http://backend")

@pytest.mark.asyncio
async def test_healthz(api):
    assert await api.healthz() == {"status": "ok"}


@pytest.mark.asyncio
async def test_eval_success(api):
    """Test the /eval endpoint with a mock backend response."""
    mock_response = {"result": "mocked response"}

    with patch.object(api.client, "post", new_callable=AsyncMock) as mock_post:
        mock_post.return_value.json = AsyncMock(return_value=mock_response)
        mock_post.return_value.raise_for_status = AsyncMock()

        response = await api.eval({"code": "(+ 1 2)"})  # Mock Lisp code

        assert await response == mock_response  # Ensure correct response

@pytest.mark.asyncio
async def test_eval_http_error(api):
    """Test /eval endpoint handling HTTP errors."""
    with patch.object(httpx.AsyncClient, "post", new_callable=AsyncMock) as mock_post:
        mock_post.side_effect = httpx.HTTPStatusError("Bad Request", request=None, response=None)

        with pytest.raises(httpx.HTTPStatusError):
            await api.eval({"code": "(+ 1 2)"})
