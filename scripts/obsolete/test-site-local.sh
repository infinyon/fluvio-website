#!/bin/bash

# script path & timestamp
DIR="$(dirname $( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd ))"

# cli help
cliHelp()
{
   echo ""
   echo "Usage: $0 -s <source-dir> -i <source-img-dir> -d <destination-folder>"
   echo -e "    -s \tsource dir           (~/projects/fluvio/docs/cli)"
   echo -e "    -i \tsource img dir       (~/projects/fluvio/docs/cli-img) - optional"
   echo -e "    -d \tdestination folder   (cli)"
   exit 1 # Exit script after printing help
}

# validate params
while getopts "s:d:i:" opt
do
   case "$opt" in
      s ) src_docs_path="$OPTARG" ;;
      i ) src_img_path="$OPTARG" ;;
      d ) dst_docs_path="$OPTARG" ;;
      ? ) cliHelp ;; # Print cliHelp in case parameter is non-existent
   esac
done

if [ -z "$src_docs_path" ] || [ -z "$dst_docs_path" ]
then
   echo "Invalid parameters";
   cliHelp
fi

# build from and to paths
from_docs="${src_docs_path}"
to_docs="${DIR}/content/docs/${dst_docs_path}"

# copy from docs
if [ -d $from_docs ] 
then
   echo "copy from files from '${from_docs}' to '${to_docs}' "
   cp -vpr $from_docs $to_docs
else
    echo "cannot file source director ${from_docs}"
fi
