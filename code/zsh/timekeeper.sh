#!/bin/zsh

message=$1
# The first item after `./timekeeper.sh`

currtime=$(date -Iseconds)
# ISO-8601 standard for time, specified down to seconds

message=($currtime ":" $message)

topic="timekeeper"
# the fluvio topic to be transmitted to

if $(fluvio topic list | grep -q "$topic")
   # fluvio topic list
   # this returns all the topics in the database
then
else
	fluvio topic create $topic
	# fluvio topic create <topic name>
	# this tells Fluvio to create a topic in the database.
	# In this case, the topic is timekeeper.

fi

	echo $message | fluvio produce $topic

	# fluvio produce <topic name>
	# this gives the order to transmit the following file to the
	# selected topic. In this case, the topic is timekeeper.

	# fluvio does not take in arguments directly, it must either be
	# read in from stdin, or be a file.
	# so we pipe the contents of the message into stdin.
