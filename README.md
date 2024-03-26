# Database Notes
## Docker setup
- Pull image:
  docker pull postgres
  
- Create persistent volume:
  docker volume create my_postgres_data
  
- Run container:
  docker run --name my-postgres-container -e POSTGRES_PASSWORD=mysecretpassword -v my_postgres_data:/var/lib/postgresql/data -d postgres
  
- Access the cli of the database in the container:
  docker exec -it my-postgres-container psql -U postgres

## Database setup
- Create:
  CREATE DATABASE mydatabase;
  
- Connect:
  \c mydatabase
  
- Create table:
  CREATE TABLE users (
      id SERIAL PRIMARY KEY,
      username VARCHAR(50) UNIQUE NOT NULL,
      email VARCHAR(100) UNIQUE NOT NULL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );
  
- Insert:
  INSERT INTO users (username, email) VALUES ('john_doe', 'john@example.com');
  
- Select:
  SELECT * FROM users;

## To stop and remove container
docker stop my-postgres-container
docker rm my-postgres-container

## Remove all data
docker volume rm my_postgres_data

For some reason docker-compose not working.
