FROM rust:1.80 as builder

ARG DYNAMODB_ENDPOINT

RUN apt-get update && apt-get install -y python3-pip
RUN pip install cargo-lambda --break-system-packages

WORKDIR /usr/src/myapp
COPY . .

ENV DYNAMODB_ENDPOINT=${DYNAMODB_ENDPOINT}

CMD ["cargo", "lambda", "start", "-a", "0.0.0.0"]

