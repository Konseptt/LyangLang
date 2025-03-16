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
    "run")
        if [ -z "$2" ]; then
            echo "Usage: ./build.sh run <filename.nbh> [--vm]"
        else
            cargo run -- "$2" ${3:-""}
        fi
        ;;
    "vm")
        if [ -z "$2" ]; then
            echo "Usage: ./build.sh vm <filename.nbh>"
        else
            cargo run -- "$2" --vm
        fi
        ;;
    *)
        echo "Usage: ./build.sh [build|test|clean|run|vm]"
        echo "  run <filename.nbh>    - Run program using the interpreter"
        echo "  vm <filename.nbh>     - Run program using the Lyangpiler VM"
        ;;
esac
