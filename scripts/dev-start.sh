#!/usr/bin/env bash
set -e

echo "Starting PostgreSQL docker container..."
docker compose up -d

echo "Running SQLx migrations..."
sqlx migrate run

echo "Seeding development database..."
docker exec -i crypto-db psql -U postgres -d crypto_portfolio < seeds/dev_seeds.sql

echo "Ready to develop! Run 'cargo run' to start the app."
