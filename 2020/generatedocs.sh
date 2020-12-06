#!/bin/bash
for days in *.ipynb; do
	jupyter nbconvert --to html $days
	jupyter nbconvert --to pdf $days
done
