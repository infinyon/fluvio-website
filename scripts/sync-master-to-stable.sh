#!/bin/sh

set -e
set -o verbose

git checkout stable
git pull

branch_date=$(date +"%Y-%m-%d-%H.%M")
branch_name="sync-to-stable--$branch_date"
git checkout -b $branch_name
git merge origin/master -s recursive -Xtheirs -m "Merge master to stable $branch_date"

gh pr create \
    -B stable \
    -H $branch_name \
    --title "merge master to stable $branch_date" --body "script run $branch_date $0 "
git checkout master

