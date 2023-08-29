# rust web

## running locally

cargo run

## using docker

docker build -t rust_web .

docker run -p 8080:8080 rust_web

## docker build debug

google image has no shell, so use debian to run ls 
 docker build --progress=plain -t rust_web .
