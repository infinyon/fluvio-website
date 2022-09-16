#!/bin/bash

message=$1
# The first item after `./timekeeper.sh`

currtime=$(date -Iseconds)
# ISO-8601 standard for time, specified down to seconds

key="MSG"

message="$key -  $currtime : $message"

topic="timekeeper-with-connector"
# the fluvio topic to be transmitted to

connector_name="cat-facts"

config="./catfact.yml"

if ! $(fluvio topic list | grep -q $topic) ; then
    # fluvio topic list:
    # This returns all the topics in the database.
    # This line checks to see if the topic does *not* exist,
    # and if that is the case, it continues with the next line
    # of code.

    fluvio topic create $topic
    # fluvio topic create <topic name>:
    # This tells Fluvio to create a topic (timekeeper-with-connector)
    # in the database.

fi

if ! $(fluvio connector list | grep -q $connector_name) ; then
    # fluvio connector list:
    # This returns all connectors currently set up in Fluvio.
    # This line checks to see if the connector (cat-facts) exists.
    # If that is not the case, it executes the next line and creates it.
    
    fluvio connector create --config=$config
    # fluvio connector create --config <config file>:
    # This tells Fluvio to create a connector using the yml
    # config file (catfact.yml) provided.

fi

echo $message | fluvio produce $topic --key-separator=" - "
# fluvio produce <topic name> --key-separator=<key>:
# This gives the order to transmit the contents of stdin to
# the selected topic (timekeeper), using the string
# in <key> ( - ) to delineate the key from the rest of the message.
