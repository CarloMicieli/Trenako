FROM node:lts-bookworm-slim@sha256:18aacc7993a16f1d766c21e3bff922e830bcdc7b549bbb789ceb7374a6138480 as builder
WORKDIR /docs

# To solve "FATAL ERROR: Reached heap limit Allocation failed - JavaScript heap out of memory"
ENV NODE_OPTIONS="--max_old_space_size=4096"

COPY ../openapi .

RUN npm update -g npm
RUN npm install redoc-cli -g

RUN redoc-cli build openapi.yaml --options.theme.colors.primary.main=blue

FROM nginx:alpine@sha256:7e528502b614e1ed9f88e495f2af843c255905e0e549b935fdedd95336e6de8d as runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako openapi documentation"

COPY --from=builder /docs/redoc-static.html /usr/share/nginx/html/index.html
