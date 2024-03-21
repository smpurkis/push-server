# Push-Server

Push-Server is a Rust-based server application that provides push notification services. It uses the Actix-web framework for handling HTTP requests and the web-push library for sending push notifications.

## Features

- **Subscription**: Clients can subscribe to receive push notifications.
- **Push Notifications**: Send push notifications to subscribed clients.
- **Public Key Retrieval**: Retrieve the public key used for encrypting push notifications.
- **Health Check**: A simple health check endpoint.

## Setup

1. Clone the repository.
2. Navigate to the project directory.
3. Run the `generate_keys.sh` script to generate the necessary keys for the server:
    ```shell
    ./generate_keys.sh
    ```
4. Build and run the server:
    ```shell
    cargo run
    ```

## Endpoints

- `POST /subscribe`: Subscribe to push notifications. The request body should be a JSON object containing the subscription ID and subscription info.
- `POST /push-message`: Send a push notification. The request body should be a JSON object containing the subscription ID and the push notification data.
- `GET /public-key`: Retrieve the public key used for encrypting push notifications.
- `GET /health`: A simple health check endpoint.

## License

This project is licensed under the MIT License.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
