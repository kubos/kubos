#!/bin/bash

set -e

this_dir=$(cd `dirname "$0"`; pwd)
kubos_dir=$(cd "$this_dir/../.."; pwd)

latest_tag=`git tag --sort=-creatordate --sort=-v:refname| head -n 1`

echo "Tag for docs: $latest_tag"

target_doc_dir="$kubos_dir/target/doc"
if [[ ! -d "$target_doc_dir" ]]; then
    mkdir -p "$target_doc_dir"
fi

cat >"$target_doc_dir/kubos_build_info.py" <<THIS
version = u'$latest_tag'
release = u'$latest_tag'
THIS

# Place current docs tag into docs conf
#sed -i "s/0.0.0/$latest_tag/g" docs/conf.py

echo "Generating new docs"
./tools/docs/gendocs.py --version $latest_tag

chmod a+rw -R html/
