#!/bin/bash

function generateProtobuf() {
    # shellcheck disable=SC2164
    # cd xproto/proto
    # # shellcheck disable=SC2035
    # protoc --go_out=. --go-grpc_out=. *.proto
    # git add .
    echo "test"
}

function dockerBuild() {
    current_time=$(date +"%Y%m%d%H%M")
    docker build -t cc360428:rust-web-"$current_time" .
}

if [ "$#" -eq 0 ]; then
    generateProtobuf
else
    case $1 in
    1)
        generateProtobuf
        ;;
    2)
        dockerBuild
        ;;
    3)
        codeCheck
        ;;
    esac
fi

exit 0
