# Use an official Rust runtime as a parent image
FROM rust:1.54 as builder

# Set the working directory in the container to /usr/src/app
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Install any needed packages specified in Cargo.toml
RUN cargo install --path .

# Start a new stage. This is for the runtime
FROM debian:buster-slim

# Copy the binary from builder to this new stage
COPY --from=builder /usr/src/app/target/release/main /usr/local/bin/main

# Make port 1236 available to the world outside this container
EXPOSE 1236

# Run the binary program produced by `cargo install`
CMD ["main"]


# Build the Docker image
# docker build -t push-server .

# Run the Docker container
# docker run -p 1236:1236 push-server