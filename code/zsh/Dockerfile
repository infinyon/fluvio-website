# This image is for CI testing purposes only
FROM ubuntu:latest
# Installing Fluvio CLI for creating topics in example

# setting up for installing zsh
RUN apt-get update
RUN apt-get install -y zsh

COPY ./timekeeper.sh .
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | sh
RUN ls -alh | echo
RUN /bin/zsh timekeeper.sh 'hello world!'