#!/bin/bash

CURRENT_DIR=`pwd`
TARGET_DIR=/webapps/nieufnipl
SSH_TARGET=root@165.227.133.227

BACKEND_SUBDIR="nieufnibackend/target/release/nieufnibackend"


TARGET_BACKEND="$TARGET_DIR/$BACKEND_SUBDIR"
TARGET_FRONT="$TARGET_DIR/nieufnifront/dev"

ARICLES_DIR="$CURRENT_DIR/nieufnibackend/articles"
MIGRATIONS_DIR="$CURRENT_DIR/nieufnibackend/migrations"

LOCAL_BACKEND_BINARY="$CURRENT_DIR/$BACKEND_SUBDIR"
LOCAL_COMPILED_FRONT="$CURRENT_DIR/nieufnifront/dist"

echo ${TARGET_FRONT}
echo ${TARGET_BACKEND}

cd ./nieufnifront/ && \
yarn build && \
cd ${CURRENT_DIR} && \
cd ./nieufnibackend/ && \
cargo build --release && \
echo "creating directory structure..." && ssh ${SSH_TARGET} mkdir -p ${TARGET_FRONT} && ssh ${SSH_TARGET} mkdir -p ${TARGET_BACKEND};
echo "cleaning up..." && ssh ${SSH_TARGET} rm -rf ${TARGET_FRONT} || ssh ${SSH_TARGET} rm -rf ${TARGET_BACKEND};
echo "scp -rp ${LOCAL_COMPILED_FRONT}/* ${SSH_TARGET}:${TARGET_FRONT}" && scp -rp ${LOCAL_COMPILED_FRONT} ${SSH_TARGET}:${TARGET_FRONT} && \
echo "copying backend..." && ssh ${SSH_TARGET} systemctl stop nieufnipl && scp ${LOCAL_BACKEND_BINARY} ${SSH_TARGET}:${TARGET_DIR}/nieufnibackend/ && \
echo "copying articles..." && scp -r ${ARICLES_DIR} ${SSH_TARGET}:${TARGET_DIR}/nieufnibackend && \
echo "copying migrations..." && scp -r ${MIGRATIONS_DIR} ${SSH_TARGET}:${TARGET_DIR}/nieufnibackend && \
echo "restarting..." && ssh ${SSH_TARGET} systemctl restart nieufnipl && \
echo "done!"
