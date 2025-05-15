#!/bin/sh

wget -O PW_OpenAPI.yaml https://raw.githubusercontent.com/Pirate-Weather/pirateweather/refs/heads/main/PW_OpenAPI.yaml
npx @openapitools/openapi-generator-cli generate -i PW_OpenAPI.yaml -g rust --package-name=pirate_weather --library reqwest-trait -o .
cargo fmt --all