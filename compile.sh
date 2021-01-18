#!/bin/bash

SCHEMA_PATH="${PWD}/schemas"
DIST_RUST_PATH="${PWD}/rust"
DIST_GO_PATH="${PWD}/go"

function compile() {
    local language=$1
    local schema_path=$2
    local dist_path=$3
    local suffix=".txt"

    case $1 in
    rust) suffix=".rs" ;;
    go)   suffix=".go" ;;
    esac

    if [[ ! -d $dist_path ]]; then
        mkdir -p $dist_path
    fi

    # walk through all directories recursively
    files=$(ls -a $schema_path)
    for file in $files; do
        if [[ $file != .* ]]; then
            if [ ! -d "${file}" ]; then
                echo "Compile ${schema_path}/${file} to ${dist_path}/${file%.*}${suffix}"
                moleculec --language $language --schema-file ${schema_path}/${file} >${dist_path}/${file%.*}${suffix}
            else
                compile $language $schema_path $dist_path
            fi
        fi
    done
}

case $1 in
rust)
    compile rust $SCHEMA_PATH $DIST_RUST_PATH/src/schemas
    cd $DIST_RUST_PATH
    cargo fmt
    ;;
go)
    compile go $SCHEMA_PATH $DIST_GO_PATH/src
    ;;
*)
    echo "Unsupported compiling target."
    exit 0
    ;;
esac

echo "Done ✔"
