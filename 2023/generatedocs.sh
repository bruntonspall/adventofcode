#!/bin/bash
. venv/bin/activate

for days in *.ipynb; do
echo ${days/ipynb/html}
	if [[ ! -e ${days/ipynb/html} ]]; then
		jupyter nbconvert --to html "$days"
	fi
done

cat > index.md <<EOF
# Advent of Code 2021
This is my set of entries for [advent of code 2023](https://www.adventofcode.com/2023).

## Completed days
  So far I've completed the following days:

EOF
for days in *.html; do
	echo "  * [${days/.html/}]($days)" >> index.md
done;
