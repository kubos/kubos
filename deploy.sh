#! /bin/bash

latest_tag=`git tag --sort=-creatordate | head -n 1`
new_build_field="+1"

echo "Latest tag: $latest_tag"

#Stable releases follow major.minor.patch version format
major_regex='^[0-9]+\.[0-9]+\.[0-9]+$'
#Unstable (CD) releases follow major.minor.patch+build version format
build_regex='^[0-9]+\.[0-9]+\.[0-9]+\+[0-9]+$'

if [[ $latest_tag =~ $major_regex ]];
then
    echo "The latest tag is a stable release. Adding a 'build' field to the version..."
    new_tag="$latest_tag$new_build_field"
elif [[ $latest_tag =~ $build_regex ]];
then
    echo "The latest tag is a CI/CD release. Incrementing the build version field..."
    version_list=(`echo $latest_tag | tr '.' ' '`)
    v_major=${version_list[0]}
    v_minor=${version_list[1]}
    # we have to split the patch and build versions on "+" instead of a "."
    pathAndBuild=(`echo ${version_list[2]} | tr '+' ' '`)
    v_patch=${pathAndBuild[0]}
    v_build=${pathAndBuild[1]}
    ((v_build+=1))
    new_tag="$v_major.$v_minor.$v_patch+$v_build"
fi

echo "Tagging new verson: $new_tag"
git tag $new_tag

echo "Pushing new tag..."
git push origin $new_tag
