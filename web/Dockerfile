FROM node:23-alpine AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

COPY ./ ./

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

RUN pnpm run build

FROM nginx:1.27.4-alpine-slim

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=base /app/dist /usr/share/nginx/html

COPY env.sh /docker-entrypoint.d/35-provide-runtime-env.sh
RUN chmod +x /docker-entrypoint.d/35-provide-runtime-env.sh

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
