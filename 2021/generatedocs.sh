#!/bin/bash
. venv/bin/activate

for days in *.ipynb; do
echo ${days/ipynb/md}
	if [[ ! -e ${days/ipynb/md} ]]; then
		jupyter nbconvert --to markdown "$days"
	fi
done

