#!/bin/bash

echo -e "\nSyncing flatbuffers...\n"

if [ -z "$(which flatc)" ]; then
    echo "Please install/build the flatbuffers compiler (flatc):"
    echo "https://google.github.io/flatbuffers/flatbuffers_guide_building.html"
    exit 1
fi

cd "${0%/*}"

rm -rf ./flatbuffers
flatc --rust -o ./flatbuffers/ ../flatbuffers/*

echo -e "\nDone!\n"
