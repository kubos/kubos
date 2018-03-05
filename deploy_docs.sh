#! /bin/bash

latest_tag=`git tag --sort=-creatordate | head -n 1`

echo "Uploading new docs.."
git clone https://github.com/kubos/pm-tools
./pm-tools/docs/upload.py $latest_tag ./html/
