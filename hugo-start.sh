#!/bin/bash
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
--renderToDisk \
#--port 1313
