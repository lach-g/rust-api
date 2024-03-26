#!/bin/bash

echo "Pulling Postgres Docker image"
docker pull postgres

echo "Create data volume if not existing"
docker volume create data

echo "Running db-container"
docker run \
  --name db-container \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  -v data:/var/lib/postgresql/data \
  -v ./sql/create_tables.sql:/docker-entrypoint-initdb.d/create_tables.sql \
  -d postgres

echo "Access via cli: docker exec -it db-container psql -U user"
