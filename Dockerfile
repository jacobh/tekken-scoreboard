FROM node:7

RUN curl -o- -L https://yarnpkg.com/install.sh | bash
RUN chmod 777 ~/.yarn/bin/yarn

WORKDIR /app

ADD package.json package.json
ADD yarn.lock yarn.lock
RUN yarn

ADD backend/package.json backend/package.json
ADD backend/yarn.lock backend/yarn.lock
RUN (cd backend && ~/.yarn/bin/yarn)

ADD frontend/package.json frontend/package.json
ADD frontend/yarn.lock frontend/yarn.lock
RUN (cd frontend && ~/.yarn/bin/yarn)

COPY . .

RUN (cd frontend && ~/.yarn/bin/yarn run build)

CMD (cd backend && /root/.yarn/bin/yarn run dev)
EXPOSE 4000