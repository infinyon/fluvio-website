# This image is for CI testing purposes only
FROM python:3
# Installing Fluvio CLI for creating topics in example
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | bash
ENV PATH=/usr/local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.fluvio/bin
RUN pip install fluvio
COPY hello-python.py .
COPY patch-uploader.py .
COPY ./testfiles/test .test
CMD python hello-python.py

CMD fluvio topic create patch-autosave
CMD fluvio produce patch-autosave -f test
CMD python patch-uploader.py