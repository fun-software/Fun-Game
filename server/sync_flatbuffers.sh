#!/bin/bash

echo -e "\nSyncing flatbuffers...\n"

if [ -z "$(which flatc)" ]; then
    echo "Please install/build the flatbuffers compiler (flatc):"
    echo "https://google.github.io/flatbuffers/flatbuffers_guide_building.html"
    exit 1
fi

cd "${0%/*}"

# Remove all files in ./src/fbs except mod.rs, then generate new fbs definitions
find ./src/fbs ! -name 'mod.rs' -type f -exec rm -f {} +
flatc --rust --filename-suffix '' --gen-object-api --gen-compare --include-prefix 'fbs' \
    -o ./src/fbs/ $(find ../flatbuffers -name "*.fbs")

echo -e "\nDone!\n"
