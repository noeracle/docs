# docs.noeracle.org (Docusaurus; content GitBook-synced into ../docs) for
# Azure Container Apps. Repo-root context — docs_website reads ../docs.
# Build: az acr build -r noetheracr2026 -f Dockerfile -t noeracle-docs:vN .
FROM node:22-alpine AS builder
WORKDIR /repo
COPY . .
WORKDIR /repo/docs_website
RUN npm ci
RUN npm run build

FROM nginx:alpine
COPY --from=builder /repo/docs_website/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
