FROM debian:bookworm-slim

ADD . /app/
WORKDIR /app

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# Install requirements
RUN uv sync --frozen

# Run the app
CMD ["uv", "run", "main.py"]
