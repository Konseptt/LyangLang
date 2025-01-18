#!/bin/bash

case $1 in
    "build")
        cargo build
        ;;
    "test")
        cargo test
        ;;
    "clean")
        cargo clean
        ;;
    *)
        echo "Usage: ./build.sh [build|test|clean]"
        ;;
esac
