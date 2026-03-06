FROM oven/bun:latest

WORKDIR /app

# Copy only package files first for better layer caching
COPY package.json bun.lock ./

# Install dependencies (this layer is cached if package files don't change)
RUN bun install --frozen-lockfile

# Copy source files
COPY . ./

# Build the Astro site
RUN bun run build

CMD ["bun", "./dist/server/entry.mjs"]