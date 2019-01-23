#!/bin/bash

CURRENT_DIR=`pwd`
TARGET_DIR=/webapps/nieufnipl

cd ./nieufnifront/ && \
yarn build && \
cd ${CURRENT_DIR} && \
echo "cleaning up" && ssh root@165.227.133.227 rm -rf ${TARGET_DIR}/* && \
echo "copying..." && scp -rv ./* root@165.227.133.227:${TARGET_DIR} && \
echo "running..." && ssh root@165.227.133.227 bash -c "cd $TARGET_DIR && ./ignite.sh" && \
echo "done!"
