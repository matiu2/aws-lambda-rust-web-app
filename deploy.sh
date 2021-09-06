#!/bin/sh
mkdir deploy
cargo build --release && cp target/release/lambda-web deploy/bootstrap && cd deploy && zip out.zip bootstrap && aws lambda update-function-code --function-name rust-web --zip-file fileb://out.zip && cd ..
