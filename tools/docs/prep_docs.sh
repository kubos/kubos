#!/bin/bash

set -e

latest_tag=`git tag --sort=-creatordate --sort=-v:refname| head -n 1`

echo "Tag for docs: $latest_tag"

# Place current docs tag into docs conf
sed -i "s/0.0.0/$latest_tag/g" docs/conf.py

echo "Generating new docs"
pushd tools
poetry install --no-interaction --no-ansi
poetry run ./docs/gendocs.py --version $latest_tag
popd

chmod a+rw -R html/
