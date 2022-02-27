FROM node:16-alpine as builder

WORKDIR /app

ENV PATH /app/node_modules/.bin:$PATH

COPY todos/package.json ./
COPY todos/package-lock.json ./
RUN npm install --silent

ADD todos/src ./src
ADD todos/public ./public

RUN npm run build

FROM nginx:stable-alpine
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]