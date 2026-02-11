FROM node:24-alpine AS builder

WORKDIR /web-client
COPY . .

RUN yarn install && yarn build


FROM nginx:1.28-alpine AS server

RUN rm -rf /usr/share/nginx/html/*
COPY --from=builder /web-client/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/nginx.conf
RUN chmod -R 755 /usr/share/nginx/html

EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]
