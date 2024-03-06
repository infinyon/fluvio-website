#!/bin/sh

set -e

git checkout stable
git pull

branch_date=$(date +"%Y-%m-%d-%H.%M")
git checkout -b sync-to-stable--$branch_date
git merge origin/master -s recursive -Xtheirs -m "Merge master to stable $branch_date"

gh pr create -B stable --title "merge master to stable $branch_date" --body "script run $branch_date $0 "
git checkout master

