FROM jenkins/jenkins:2.387.1-lts

USER root

RUN apt-get update &&\
	apt-get install curl git less vim -y

ARG JENKINS_HOME CONTAINER_HOME
ARG COPY_REFERENCE_LOG_FILE_PATH
ENV JENKINS_HOME=${JENKINS_HOME} \
    COPY_REFERENCE_LOG_FILE_PATH=${COPY_REFERENCE_LOG_FILE_PATH} \
    CONTAINER_HOME=${CONTAINER_HOME}

ADD ./build-dependencies $CONTAINER_HOME

RUN mkdir -p $JENKINS_HOME &&\
    chown "jenkins:jenkins" $JENKINS_HOME

RUN echo "Setting up the container file system"
RUN $CONTAINER_HOME/build-scripts/setup-fs.sh

USER jenkins

RUN echo "Configuring the jenkins instance"
RUN $CONTAINER_HOME/build-scripts/configure-jenkins.sh
