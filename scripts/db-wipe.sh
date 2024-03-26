#!/bin/bash

echo "Stopping db-container"
docker stop db-container
echo "Removing db-container"
docker container rm db-container
echo "Removing data volume"
docker volume rm data
