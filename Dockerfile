# This Dockerfile was tested on Arch Linux, and splits the build into 3 stages:
# 1. Install the Oracle Instant Client, this is done using the 3 RPM files provided by Oracle
# and stored with the directory containing this Dockerfile.
# 2. Install libaio, this is required by the Oracle Instant Client.
# 3. Build the Rust application, this is done using the official Rust Docker image.
# The Oracle Instant Client and libaio are copied from the first 2 stages to the last stage.
# The LD_LIBRARY_PATH environment variable is set to the Oracle Instant Client library path.
# TODO: Test on Windows and macOS
# TODO (maybe): Use a multi-stage build to reduce the size of the final image
# TODO (maybe): Clean up instantclient installation


# Stage 1: Install the Oracle Instant Client
FROM oraclelinux:7-slim AS instantclient

ADD oracle-instantclient*.rpm /tmp/

RUN  yum -y install /tmp/oracle-instantclient*.rpm && \
     rm -rf /var/cache/yum && \
     rm -f /tmp/oracle-instantclient*.rpm && \
     echo /usr/lib/oracle/12.2/client64/lib > /etc/ld.so.conf.d/oracle-instantclient12.2.conf && \
     ldconfig

# Stage 2: Install libaio and copy it to the last stage
FROM ubuntu:latest AS libaio

RUN apt-get update && \
    apt-get install -y libaio-dev && \
    rm -rf /var/lib/apt/lists/*

# Stage 3: Build the Rust application
FROM rust:latest as final

ENV MAGICK_VERSION 7.1

RUN curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION}* \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}*

COPY --from=instantclient /usr/lib/oracle /usr/lib/oracle
COPY --from=libaio /usr/lib/x86_64-linux-gnu/libaio.so /usr/lib/x86_64-linux-gnu/libaio.so.1

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app


# Set environment variables for OpenSSL and pkg-config
ENV OPENSSL_DIR=/usr/lib/ssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl

# Install libssl-dev equivalent package for the Rust image
RUN apt-get update && \
    apt-get install -y libssl-dev libjpeg-turbo-progs libpng-dev clang llvm && \
    rm -rf /var/lib/apt/lists/*

ENV LD_LIBRARY_PATH=/usr/lib/clang:$LD_LIBRARY_PATH
ENV LD_LIBRARY_PATH=/usr/local/lib

RUN ldconfig /usr/local/lib
RUN rm /usr/lib/x86_64-linux-gnu/libMagick*

ENV IMAGE_MAGICK_DIR=/usr/local
ENV LD_LIBRARY_PATH=/usr/lib/oracle/12.2/client64/lib:$LD_LIBRARY_PATH

# Build the Rust application
RUN cargo build --release

# Expose port 8000 if needed
EXPOSE 8000

# Set the entry point to run the program
CMD ["./target/release/jhapi"]
