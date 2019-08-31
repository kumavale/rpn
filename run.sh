#!/bin/bash

cargo build

if [ $? -ne 0 ]; then
    exit 1
fi

function runtest() {
    ./target/debug/rpn "$1" "$2"
}

echo '=== arithmetic operators  ==='
runtest '1 2 +' 3
runtest '100 5 +' 105
runtest '5 1 -' 4
runtest '1 4 -' -3
runtest '3 5 *' 15
runtest '20 5 /' 4
runtest '1 2 3 4 + + +' 10
runtest '4 3 + 2 + 1 +' 10
runtest '3 4 * 2 +' 14

