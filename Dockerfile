# Build frontend
FROM node:18-alpine AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# Build Rust backend
FROM rust:1.75-slim AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies for KataGo
RUN apt-get update && apt-get install -y \
    libzip4 \
    libgomp1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/go-server /app/go-server

# Copy frontend build
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Copy KataGo assets
COPY assets/ /app/assets/

# Expose port
EXPOSE 3000

# Run
CMD ["/app/go-server"]
