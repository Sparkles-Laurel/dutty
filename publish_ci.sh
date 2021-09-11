#!/usr/bin/bash
for item in $(ls)
do
	git add ./$item
done

git commit -m "$*"
git push
cargo publish
