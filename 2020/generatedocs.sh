#!/bin/bash
for days in *.ipynb; do
echo ${days/ipynb/html}
	if [ ! -e ${days/ipynb/html} ]; then
		jupyter nbconvert --to html $days
	fi
	if [ ! -e ${days/ipynb/pdf} ]; then
		jupyter nbconvert --to pdf $days
	fi
done
