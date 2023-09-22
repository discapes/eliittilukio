FROM node:alpine
WORKDIR /build

COPY package*.json ./
RUN npm i
COPY . .
RUN npx svelte-kit sync
RUN npm run build


FROM node:alpine
WORKDIR /app
EXPOSE 8888
ENV PORT=8888

RUN echo '{"type":"module"}' > package.json 
COPY --from=0 /build/build/ .
CMD node .