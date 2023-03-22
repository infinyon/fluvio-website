#!/bin/bash
WAIT=20

printf "\nThis script is deprecated.\nInstead run:\ncargo run -- hugo"

if [[ -z "$CI" ]]; then
    printf "\n\nStarting in $WAIT seconds..."
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
