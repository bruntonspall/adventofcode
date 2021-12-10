#!/bin/sh
docker build -t adventofcode2021 .
docker run -it --rm -p 8888:8888 -v ${PWD}:/app adventofcode2021
