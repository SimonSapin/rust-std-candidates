#!/bin/sh
set -e
cd $(dirname $0)
for lib in */
do
    cd $lib
    cargo "$@"
    cd ..
done
