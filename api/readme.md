# rust web

## running locally

cargo run

## using docker

```shell
docker build -t api .
docker run -p 8080:8080 api
```

## docker build debug

google image has no shell, so use debian to run ls 

```shell
 docker build --progress=plain -t api .
```