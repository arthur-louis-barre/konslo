FROM node:24-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
ARG BUILD_CONFIG=production
RUN npm run build -- --configuration $BUILD_CONFIG

FROM nginx:alpine AS runtime
COPY --from=builder /app/dist/konslo-client/browser /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80