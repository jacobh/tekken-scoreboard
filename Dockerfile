FROM node:7

RUN curl -o- -L https://yarnpkg.com/install.sh | bash

WORKDIR /app

ADD package.json package.json
RUN yarn

ADD backend/package.json backend/package.json
RUN (cd backend && ~/.yarn/bin/yarn)

ADD frontend/package.json frontend/package.json
RUN (cd frontend && ~/.yarn/bin/yarn)

COPY . .

RUN (cd frontend && ~/.yarn/bin/yarn run build)

CMD (cd backend && ~/.yarn/bin/yarn run dev)
EXPOSE 4000