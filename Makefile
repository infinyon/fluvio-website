UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
NPROCS = $(shell grep -c 'processor' /proc/cpuinfo)
MAKEFLAGS += -j$(NPROCS)
endif
ifeq ($(UNAME), Darwin)
NPROCS = $(shell sysctl hw.ncpu  | grep -o '[0-9]\+')
MAKEFLAGS += -j$(NPROCS)
endif

CODE_ROOT=embeds
CLIENT_EXAMPLE_ROOT=$(CODE_ROOT)/client-examples


cluster-verify:
	fluvio version
	fluvio topic list
	fluvio topic create foobar
	sleep 30
	echo foo | fluvio produce foobar
	fluvio consume foobar -B -d
	fluvio topic delete foobar;

htmltest:
	curl https://htmltest.wjdp.uk | bash
	./bin/htmltest

run-client-example:
	cd $(CLIENT_EXAMPLE_ROOT)/$(LANG); \
	docker-compose build; \
	docker-compose run example;

run-client-example-rust: LANG=rust
run-client-example-rust: run-client-example

run-client-example-node: LANG=node
run-client-example-node: run-client-example

run-client-example-java: LANG=java
run-client-example-java: run-client-example

run-client-example-python: LANG=python
run-client-example-python: run-client-example

