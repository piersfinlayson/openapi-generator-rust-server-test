# openapi-generator-rust-server-test
A test of openapi-generator rust-server output

## Using

```
cd ~
git clone https://github.com/piersfinlayson/openapi-generator-rust-server-test
cd openapi-generator-rust-server-test
docker pull openapitools/openapi-generator-cli
docker run --rm \
           -v ~/openapi-generator-rust-server-test/openapi_client:/local \
           openapitools/openapi-generator-cli \
           generate \
           -g rust-server \
           -i /local/openapi.yaml \
           -o /local
cd openapi_client
cargo build
```

