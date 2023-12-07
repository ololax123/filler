docker build -t filler .
docker run -v "$(pwd)/":/filler/solution -it filler