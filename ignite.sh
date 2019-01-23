#!/usr/bin/env bash
CURRENT_DIR=`pwd`

cd ./nieufnifront/
yarn install && \
yarn build && \
cd ${CURRENT_DIR} && \
cd ./nieufnibackend/ && \
cargo run
