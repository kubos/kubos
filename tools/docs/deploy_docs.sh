#! /bin/bash
this_dir=$(cd "`dirname "$0"`"; pwd)
repo_dir=$(cd "$this_dir/../.."; pwd)

latest_tag=`git tag --sort=-creatordate --sort=-v:refname | head -n 1`

#Unstable (CD) releases follow major.minor.patch+build version format
build_regex='^[0-9]+\.[0-9]+\.[0-9]+\+[0-9]+$'

if [[ ! -d "$repo_dir/html" ]]; then
  echo "No generated docs in $repo_dir/html" 1>&2
  exit 1
fi

tmp_dir=$(mktemp --tmpdir -d "kubos-docs.XXXXXXXXXX")

mv "$repo_dir/html" "$tmp_dir/$latest_tag"

dest_dir="/var/docs.kubos.co"
if [[ $latest_tag =~ $build_regex ]]; then
  dest_dir+="/master"
fi

echo "Uploading new docs ($latest_tag) to $dest_dir.."
tar -C "$tmp_dir" -cf - . | ssh www-data@docs.kubos.com "cd $dest_dir && tar xpvf -"
rm -rf "$tmp_dir"
