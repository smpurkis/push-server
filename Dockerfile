# Use an official Rust runtime as a parent image
FROM --platform=linux/arm64 rust:1.76 as builder

# Set the working directory in the container to /usr/src/app
WORKDIR /app

# Copy the current directory contents into the container at /usr/src/app
RUN git clone https://github.com/smpurkis/push-server.git

# Change the working directory to the app
WORKDIR /app/push-server

# Install any needed packages specified in Cargo.toml
RUN cargo build --release -j 3

# Start a new stage. This is for the runtime
FROM ubuntu:latest as runtime

WORKDIR /app

# Install the necessary dependencies
RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder to this new stage
COPY --from=builder /app/push-server/target/release/push-server /app

# Make port 1236 available to the world outside this container
EXPOSE 1236

# Run the binary
CMD ["./push-server"]

# Build the Docker image
# docker build -t push-server .

# Run the Docker container
# docker run -p 1236:1236 push-server