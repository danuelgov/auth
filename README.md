# Auth

IDaaS for Danuel Government, providing a cloud-based service for user identification and authentication, simplifying user management across various applications and services while enhancing security.

## Features

1. User registration and management
2. Authentication and authorization
3. Support for multi-factor authentication (2FA)
4. User session management
5. Security event monitoring and notification
6. API and SDK provision

## Tech Stack

- Languages: Rust, TypeScript
- Frameworks: Rocket, Next.js
- Database: MySQL
- Cloud Platform: AWS

## Installation and Execution

1. Clone this repository.
2. Navigate to the server folder and build it with `cargo build`.
3. Navigate to the client folder and install necessary packages with `yarn`.
4. To run the server, execute `cargo run --package auth_{server_name}`.
5. To run the client, execute `yarn dev` or `yarn start`.

## Contribution Guidelines

1. Fork this repository.
2. Create a new branch for the feature or bug fix: `git checkout -b feature/feature-name` or `git checkout -b bugfix/bug-name`.
3. Commit your changes: `git commit -m 'Description of added feature'`.
4. Push your branch to the remote repository: `git push origin feature/feature-name`.
5. Create a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
