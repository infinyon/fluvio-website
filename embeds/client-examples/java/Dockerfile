FROM openjdk:11

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio 

# Installing Fluvio CLI for creating topics in example
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | bash
ENV PATH=/usr/local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/fluvio/.fluvio/bin

# Copy your Java project and run it
COPY --chown=$USER:$USER fluvio-java .
CMD ./gradlew run