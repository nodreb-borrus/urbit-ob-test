FROM node:18-bookworm-slim AS base

# Install dependencies only when needed
FROM base AS deps
WORKDIR /app
# Install dependencies based on the preferred package manager
COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* ./
RUN \
  if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
  elif [ -f package-lock.json ]; then npm ci; \
  elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i --frozen-lockfile; \
  else echo "Lockfile not found." && exit 1; \
  fi

FROM rust:1.73-slim-bookworm AS builder
WORKDIR /usr/src/urbit-rs
COPY src/ ./src/
COPY Cargo.toml Cargo.lock .
RUN cargo build --locked --release \
  && cargo install --path .

FROM base AS runner
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY --from=builder /usr/local/cargo/bin/urbit-ob-test .
COPY index.js .
CMD ./urbit-ob-test
