#! /bin/bash

latest_tag=`git tag --sort=-creatordate --sort=-v:refname| head -n 1`

echo "Uploading new docs.."
# git clone https://github.com/kubos/pm-tools
# ./pm-tools/docs/upload.py $latest_tag ./html/

[ ! -z "$GH_NAME" ] && git config user.name "$GH_NAME"
[ ! -z "$GH_EMAIL" ] && git config user.email "$GH_EMAIL"

MAIN_BRANCH=$(git symbolic-ref --short HEAD)

git stash
git branch --delete --force gh-pages
git checkout --orphan gh-pages
git add -f ./html/
git commit -m "Deploy docs (commit '$latest_tag' )to GitHub pages [ci skip]"
# git filter-branch -f --prune-empty --subdirectory-filter ./html/ &&
git push -f origin gh-pages
git checkout $MAIN_BRANCH
git stash apply || :
