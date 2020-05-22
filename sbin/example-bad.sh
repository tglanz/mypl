#!/bin/bash

cargo run -- \
    -i resources/code-examples/a.mypl \
    -i resources/code-examples/b.mypl \
    -i resources/ \
    -i some-path-that-does-not-exist