# openjdk:11 is deprecated, openjdk is no longer updating builds
# eclipse-temurin was pointed to as a replacement
FROM eclipse-temurin:17

RUN apt-get update -y
RUN apt-get install -y unzip

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio

# Installing Fluvio CLI for creating topics in example
RUN curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
ENV PATH=/usr/local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/fluvio/.fluvio/bin

# Copy your Java project and run it
COPY --chown=$USER:$USER fluvio-java .
CMD ./gradlew run