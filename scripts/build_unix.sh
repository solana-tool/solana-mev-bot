#!/usr/bin/env bash
set -e
# Build Rust core
cargo build --release --manifest-path core/Cargo.toml

# Build Go dashboard
cd dashboard
GOOS=linux GOARCH=amd64 go build -o ../dist/dashboard
cd ..

# Copy python files
mkdir -p dist/python
cp analytics/*.py dist/python/

# Package
mkdir -p dist/mevbot_package
cp target/release/core dist/mevbot_package/core
cp dist/dashboard dist/mevbot_package/dashboard
cp scripts/run_mevbot.sh dist/mevbot_package/
cd dist
zip -r mevbot_package.zip mevbot_package
echo "Packaged at dist/mevbot_package.zip"
