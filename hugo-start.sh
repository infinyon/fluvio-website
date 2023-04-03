#!/bin/bash
WAIT=5

printf "\n**********\n* This script is deprecated!\n* Instead run:\n* cargo run -- hugo\n**********\n"

if [[ -z "$CI" ]]; then
    printf "\nStarting in $WAIT seconds..."
    sleep $WAIT
fi

hugo server \
    --watch \
    --verbose \
    --buildDrafts \
    --cleanDestinationDir \
    --disableFastRender \
    --buildFuture \
    --ignoreCache \
    --baseURL http://localhost \
    --appendPort \
    --navigateToChanged \
    --renderToDisk
#--port 1313
