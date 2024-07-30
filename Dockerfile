FROM rust:1.80 as builder

RUN apt-get update && apt-get install -y python3-pip
RUN pip install cargo-lambda --break-system-packages

WORKDIR /usr/src/myapp
COPY . .

CMD ["cargo", "lambda", "start", "-a", "0.0.0.0"]

