# rust web

## running locally

cargo run

## using docker

```shell
docker build -t web .
docker run -p 8080:8080 web
```

## docker build debug

google image has no shell, so use debian to run ls 

```shell
 docker build --progress=plain -t web .
```


## Frontend

```shell
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | sh

nvm install node

curl -fsSL https://get.pnpm.io/install.sh | sh -

pnpm install

pnpm run build
```