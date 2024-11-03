docker build -t pg-mockall .
docker run -d --name pg-mockall-container -p 5433:5432 pg-mockall