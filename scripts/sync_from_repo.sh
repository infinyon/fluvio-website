#!/bin/bash

# script path & timestamp
GIT_BASE="https://github.com/infinyon/"
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PWD=$(pwd)
TS=$(date +%s)

# cli help
cliHelp()
{
   echo ""
   echo "Usage: $0 -b <branch> -r <repo-name> -d <docs-folder>"
   echo -e "    -b \tsource branch   (master)"
   echo -e "    -r \trepo name       (fluvio-cloud)"
   echo -e "    -d \tdocs folder     (fluvio-cloud)"
   exit 1 # Exit script after printing help
}

# validate params
while getopts "b:r:d:" opt
do
   case "$opt" in
      b ) branch="$OPTARG" ;;
      r ) repo_name="$OPTARG" ;;
      d ) docs_folder="$OPTARG" ;;
      ? ) cliHelp ;; # Print cliHelp in case parameter is non-existent
   esac
done

if [ -z "$branch" ] || [ -z "$repo_name" ] || [ -z "$docs_folder" ]
then
   echo "Invalid parameters";
   cliHelp
fi

# build repo & local paths
from_repo="${GIT_BASE}${repo_name}.git"
local_path=".tmp/${repo_name}-${TS}"
to_local_path="${DIR}/../${local_path}"

# clone repo & detach from origin
echo "clone '${from_repo}' ($branch) to '${local_path}'"
git clone --single-branch --branch $branch $from_repo $to_local_path
cd $to_local_path
git remote rm origin

# copy docs to content
from_docs="${to_local_path}/website/docs"
to_docs="${DIR}/../content/docs/${docs_folder}"
if [ -d from_docs ] 
then
    echo "copy docs from '${local_path}/website/docs' to 'content/docs/${docs_folder}' "
    cp -rf $from_docs $to_docs
else
    echo "no 'website/docs' path found in ${local_path}"
fi

# copy img to static
from_img="${to_local_path}/website/img"
to_img="${DIR}/../static/img/${docs_folder}"
if [ -d from_img ] 
then
    echo "copy img from '${local_path}/website/img' to 'static/img/${docs_folder}' "
    cp -rf $from_img $to_img
else
    echo "no 'website/img' path found in ${local_path}"
fi

# stage changes
echo "git: stage changes"
git add $to_docs
git add $to_img

# check if anything to commit
to_commit=$(git diff --staged --name-only)
if [ ! -z "$to_commit" ]
then
    git commit -m "website synced with `${repo-name} (${branch})` "
else
   echo "git: nothing to commit (no changes detected)";
fi

# clean-up
echo "remove temporary directory ${local_path}"
rm -rf $to_local_path

# cd back
cd $PWD