#!/bin/bash

# Jenkins ref directories and files
readonly JENKINS_REF=/usr/share/jenkins/ref
readonly LOG_PATH=/var/log/jenkins

# Jenkins user directories and files
readonly CREDS_PATH=$CONTAINER_HOME/credentials
readonly SRC_PATH=$CONTAINER_HOME/src
readonly FG_PATH=$SRC/Fun-Game
readonly USERS_PATH=$CONTAINER_HOME/users
readonly PLUGINS_PATH=$CONTAINER_HOME/plugins
readonly CONFIG_PATH=$CONTAINER_HOME/config
