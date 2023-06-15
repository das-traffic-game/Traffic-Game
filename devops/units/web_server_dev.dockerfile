FROM node:18-alpine

ENV SETUID=1000
ENV SETGID=1000

WORKDIR /app

ENTRYPOINT npm i && chown -R $SETUID:$SETGID /root && npm run dockerdev