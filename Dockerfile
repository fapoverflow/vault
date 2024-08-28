FROM rust:slim-bullseye AS build-utils
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM rust:slim-bullseye AS build-izzy
RUN apt update && apt install -y git
ARG IZZY_VERSION="tantivy"
RUN git clone https://gitlab.com/porn-vault/izzy
#RUN cd izzy && cargo build --release
RUN cd izzy && git checkout $IZZY_VERSION && cargo build --release

FROM node:20-alpine AS base
RUN apk update && apk add --no-cache git
ARG PV_VERSION="kill-es"
RUN git clone https://gitlab.com/porn-vault/porn-vault
#RUN cd porn-vault && cargo build --release
RUN cd porn-vault && git checkout $PV_VERSION
#RUN pwd && ls -lstr
#WORKDIR /porn-vault

FROM base AS deps-env
WORKDIR /app
COPY --from=base /porn-vault/package.json /porn-vault/pnpm-lock.yaml /app/
RUN npm i -g pnpm
RUN pnpm i --frozen-lockfile

FROM base AS build-env
WORKDIR /app
COPY --from=base /porn-vault/package.json /porn-vault/pnpm-lock.yaml /app/
COPY --from=base /porn-vault/remix.config.js /app/remix.config.js
COPY --from=deps-env /app/node_modules /app/node_modules
COPY --from=base /porn-vault/. .
RUN npm i -g pnpm
RUN pnpm build
RUN pnpm prune --prod

FROM base AS run-env
WORKDIR /app
RUN apk add --no-cache ffmpeg imagemagick
# TODO: this may not be completely right
RUN sed -i 's/name="width" value="16KP"/name="width" value="32KP"/' /etc/ImageMagick-7/policy.xml && sed -i 's/name="height" value="16KP"/name="height" value="32KP"/' /etc/ImageMagick-7/policy.xml
COPY --from=base /porn-vault/package.json /app/package.json
COPY --from=base /porn-vault/config.example.json /app/config.example.json
COPY --from=base /porn-vault/graphql /app/graphql
COPY --from=base /porn-vault/locale /app/locale
COPY --from=base /porn-vault/remix.config.js /app/remix.config.js
COPY --from=build-env /app/public /app/public
COPY --from=build-env /app/dist /app/dist
COPY --from=build-env /app/build /app/build
COPY --from=build-env /app/node_modules /app/node_modules

ENV PV_CONFIG_FOLDER=/config
ENV IZZY_PATH=/app/izzy
ENV NODE_ENV=production
ENV DATABASE_NAME=production
ENV PORT=3000

#FROM registry.gitlab.com/porn-vault/porn-vault:alpine-edge AS vault
COPY --from=build-utils ./target/release/vault-utils /bin/
RUN rm -rf /config/izzy
COPY --from=build-izzy ./izzy/target/release/izzy /app/
RUN chmod 777 $IZZY_PATH
RUN ls -lstr /app/

#CMD ["npm", "start"]
ENTRYPOINT ["npm", "start"]
