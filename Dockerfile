# Stage 1: Build
FROM node:22-alpine AS build
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build && \
	npm prune --omit-dev

# Stage 2: Runtime
FROM node:22-alpine
WORKDIR /app
COPY --from=build /app/build ./build
COPY --from=build /app/package.json .
COPY --from=build /app/node_modules ./node_modules

ENV NODE_ENV=dev
ENV PORT=3000
EXPOSE 3000

CMD ["node", "build"]
