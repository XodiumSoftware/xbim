#!/bin/bash

# Gen Certs for Dev - Shell Script Version

# Create certs directory
mkdir -p target/debug/certs

# Generate private key
openssl genrsa -out target/debug/certs/key.pem 2048

# Generate self-signed certificate
openssl req -new -x509 -key target/debug/certs/key.pem -out target/debug/certs/cert.pem -days 365 -subj "/CN=localhost"

# Set appropriate permissions
chmod 600 target/debug/certs/key.pem
chmod 644 target/debug/certs/cert.pem

echo "Self-signed certificates generated in target/debug/certs/"