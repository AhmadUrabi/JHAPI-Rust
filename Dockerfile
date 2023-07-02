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
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Copy the Oracle Instant Client from stage 1 to stage 3
COPY --from=instantclient /usr/lib/oracle /usr/lib/oracle

# Copy libaio from stage 2 to stage 3
COPY --from=libaio /usr/lib/x86_64-linux-gnu/libaio.so.1 /usr/lib/x86_64-linux-gnu/libaio.so.1

# Set the LD_LIBRARY_PATH environment variable
ENV LD_LIBRARY_PATH=/usr/lib/oracle/12.2/client64/lib:$LD_LIBRARY_PATH

# Install any dependencies required by the program
RUN cargo build --release

EXPOSE 8000

# Set the entry point to run the program
CMD ["./target/release/jhapi"]