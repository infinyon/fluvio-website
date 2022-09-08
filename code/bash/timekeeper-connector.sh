#!/bin/bash

message=$1
# The first item after `./timekeeper.sh`

currtime=$(date -Iseconds)
# ISO-8601 standard for time, specified down to seconds

message="$currtime : $message"

topic="timekeeper"
# the fluvio topic to be transmitted to

if ! $(fluvio topic list | grep -q $topic) ; then
    # fluvio topic list:
    # This returns all the topics in the database.
    # This line checks to see if the topic does *not* exist,
    # and if that is the case, it continues with the next line
    # of code.

    fluvio topic create $topic
    # fluvio topic create <topic name>:
    # This tells Fluvio to create a topic in the database.
    # In this case, the topic is timekeeper.

    fluvio connector create --config=./catfact.yml
    # fluvio connector create --config <config file>:
    # Thi tells Fluvio to create a connector using the yml
    # config file provided.
    # In this case, that is the catfact.yml file in the
    # same directory

fi

echo $message | fluvio produce $topic
# fluvio produce <topic name>:
# This gives the order to transmit the following file to the
# selected topic.
# In this case, the topic is timekeeper.

# Fluvio does not take in arguments directly, it must either be
# read in from stdin, or be a file. So we pipe the contents of
# the message into stdin.
