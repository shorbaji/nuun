FROM debian:bookworm-slim

WORKDIR /app

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# Install requirements
RUN uv sync --frozen

# Copy the app
COPY . .

# Run the app
CMD ["uv", "run", "api/main.py"]
