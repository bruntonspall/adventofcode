#!/bin/sh
docker build -t adventofcode2023 .
docker run -it --rm -p 8888:8888 -v ${PWD}:/app adventofcode2023
