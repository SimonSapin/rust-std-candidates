#!/bin/sh
cd $(dirname $0)
for lib in */
do
    cd $lib
    cargo "$@"
    cd ..
done
