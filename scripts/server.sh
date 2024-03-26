docker build -t server-image . && docker run --name server-container -e API_PORT=3000 -p 3000:3000 -d server-image
