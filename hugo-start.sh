#!/bin/bash
WAIT=20

echo "Starting in $WAIT seconds
This script is deprecated.

Instead run:
cargo run -- hugo"

sleep $WAIT

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
