# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Install the Oracle Instant Client
COPY instantclient-basic-linux.x64-12.2.0.1.0.zip /opt/
RUN mkdir -p /opt/oracle \
    && cd /opt/oracle \
    && cp ../instantclient-basic-linux.x64-12.2.0.1.0.zip . \
    && unzip instantclient-basic-linux.x64-12.2.0.1.0.zip

# Set the LD_LIBRARY_PATH environment variable
ENV LD_LIBRARY_PATH=/opt/oracle/instantclient_12_2:$LD_LIBRARY_PATH

# Install any dependencies required by the program
RUN cargo build --release

# Set the entry point to run the program
CMD ["./target/release/jhapi"]