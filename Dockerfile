FROM node:7.9-alpine

WORKDIR /app

ADD package.json package.json
ADD yarn.lock yarn.lock
RUN yarn

WORKDIR /app/backend
ADD backend/package.json package.json
ADD backend/yarn.lock yarn.lock
RUN yarn

WORKDIR /app/frontend
ADD frontend/package.json package.json
ADD frontend/yarn.lock yarn.lock
RUN yarn

WORKDIR /app
COPY . .

# Build backend
RUN (cd backend && yarn run build)

# Build frontend
RUN (cd frontend && yarn run build)

CMD (cd backend && yarn run serve)
EXPOSE 4000