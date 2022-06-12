FROM openjdk:11
# Installing Fluvio CLI for creating topics in example
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | bash
ENV PATH=/usr/local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.fluvio/bin
WORKDIR /root
COPY fluvio-java .
CMD ./gradlew run