#!/bin/sh

npx @openapitools/openapi-generator-cli generate -i PW_OpenAPI.yaml -g rust --package-name=pirate_weather --library reqwest-trait -o .
cargo fmt --all