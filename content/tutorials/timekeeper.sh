#!/bin/bash

message=$1
# The first item after `./timekeeper.sh`

currtime=$(date -Iseconds)
# ISO-8601 standard for time

message=($currtime ":" $message)


topic="timekeeper"
# the fluvio topic to be transmitted to

if $(fluvio topic list | grep -q "$topic")
then
	echo $message | fluvio produce $topic

	# fluvio produce <topic name> -- gives the order to transmit the
	# following file to the selected topic. In this case, the topic is timekeeper.

	# fluvio does not take in arguments directly, it must either be
	# read in from stdin, or be a file.
	# so we pipe the contents of the message into stdin.
	exit
else
	fluvio topic create $topic
	# fluvio topic create <topic name>

	echo $message | fluvio produce $topic
	exit
fi
