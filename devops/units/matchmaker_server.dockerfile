FROM node:18-alpine

COPY ./matchmaker /app
WORKDIR /app
RUN npm i
ENTRYPOINT ["node", "/app/main.js"]
