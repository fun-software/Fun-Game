#!/bin/bash
set -e

. $CONTAINER_HOME/build-scripts/constants.sh

dirs=(
    "$CONTAINER_HOME"
    "$CREDS_PATH"
    "$SRC_PATH"
    "$USERS_PATH"
)

echo "Dirs = ${dirs[@]}"
mkdir -p "${dirs[@]}"

chown -R "jenkins:jenkins" "${dirs[@]}"
usermod -d "$JENKINS_HOME" jenkins
