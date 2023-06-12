FROM node:18-alpine AS builder

COPY ./web-app /app
WORKDIR /app
RUN npm i
RUN npm run build

FROM nginx:1.25
COPY ./devops/units/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/out /content/
