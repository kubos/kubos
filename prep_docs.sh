#!/bin/bash

latest_tag=`git tag --sort=-creatordate --sort=-v:refname| head -n 1`

echo "Tag for docs: $latest_tag"

# Place current docs tag into docs conf
sed -i "s/0.0.0/$latest_tag/g" docs/conf.py

# Add the app API to the system path so that its docs can be auto-generated
pip install ./apis/app-api/python

echo "Generating new docs"
./tools/gendocs.py --version $latest_tag

chmod a+rw -R html/
