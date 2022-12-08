#!/bin/sh

docker run -p 8080:8080 -p 8081:8081 --pull always -u $(id -u):$(id -g) -v $(pwd)/live:/data livebook/livebook
