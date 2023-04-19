#!/usr/bin/env bash

# Change directories to where this script is running
pushd "$(dirname "$0")"
# Now go to twif directory
pushd ../content/news

TEMPLATE_FILENAME=this-week-in-fluvio-0000.md.tmpl
TODAY_DATE=$(date +%Y-%m-%d)
# Read in the list of files and take in the number from last one
LATEST_ISSUE=$(ls | grep -Eo '[[:digit:]]*' | tail -n1)
# Add one (in base 10...) and pad with zeroes
NEXT_ISSUE=$((10#$LATEST_ISSUE + 1))
NEXT_ISSUE_PAD4=$(printf "%04d" $NEXT_ISSUE)
NEW_FILENAME=this-week-in-fluvio-$NEXT_ISSUE_PAD4.md

###

# Script assumes that you are working in a fork repo
# and that you have https://github.com/infinyon/fluvio-website.git as a git remote named "upstream"

# Set up git stuff
#git stash
git remote add upstream git@github.com:infinyon/fluvio-website.git
git fetch upstream
git checkout master
git rebase upstream/master
git checkout -B twif-$NEXT_ISSUE

##echo $NEW_FILENAME

while read line; do
    # Replace any placeholders in template
    line=${line/'$NEXT_ISSUE'/$NEXT_ISSUE}
    line=${line/'$TODAY_DATE'/$TODAY_DATE}

    # Build the new file line by line
    #echo $line
    echo $line >>$NEW_FILENAME
done <$TEMPLATE_FILENAME

popd
popd
