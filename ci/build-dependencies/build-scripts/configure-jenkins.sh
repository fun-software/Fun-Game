#!/bin/bash
set -e

. $CONTAINER_HOME/build-scripts/constants.sh

echo "Templatizing jenkins"

echo "Installing plugins"
jenkins-plugin-cli --plugin-file $PLUGINS_PATH/requirements.txt

echo "Configuring users"
mv $USERS_PATH $JENKINS_REF/users

echo "Configuring jcasc"
mv $CONFIG_PATH $JENKINS_REF/config

echo "Configuring jobs"
mv $JOBS_PATH $JENKINS_REF/jobs
