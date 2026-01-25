
###############################################################################
## Client builder
###############################################################################
FROM node:22-slim AS client-builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
WORKDIR /client-builder
COPY ./frontend .
RUN --mount=type=cache,id=pnpm,target=/pnpm/store CI=true pnpm install --frozen-lockfile
RUN pnpm run build

###############################################################################
## Server builder
###############################################################################
FROM rust:alpine3.22 AS server-builder
RUN apk add --update --no-cache \
            autoconf \
            gcc \
            gdb \
            git \
            libdrm-dev \
            libepoxy-dev \
            make \
            mesa-dev \
            strace \
            openssl \
            openssl-dev \
            musl-dev && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists

WORKDIR /server-builder
COPY ./backend .
ENV OPENSSL_LIB_DIR=/usr/lib \
    OPENSSL_STATIC=1
RUN cargo build --release --locked

###############################################################################
## Final image
###############################################################################
FROM alpine:3.22

ENV USER=app \
    UID=1000

RUN apk add --update --no-cache \
            font-noto-emoji~=2 \
            fontconfig~=2.15 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists && \
    mkdir -p /app/static/uploads

# Copy our build
COPY --from=server-builder /server-builder/target/release/backend /app
COPY --from=client-builder /client-builder/dist/ /app/static/
COPY ./backend/migrations /app/migrations/

# Create the user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}" && \
    chown -R app:app /app && \
    fc-cache -f

WORKDIR /app
USER app
EXPOSE 3000

CMD [ "/app/backend" ]
