# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY ./src/main.rs .

# Build the Rust code inside the container
RUN rustc main.rs

# Set the command to run when the container starts
CMD ["./main"]