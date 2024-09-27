# Token Manipulation Smart Contract

This smart contract provides functionality for managing user profiles and token manipulation on the Internet Computer. It includes functions for creating user profiles, assigning tokens, transferring tokens, burning tokens, and minting new tokens.

## Features

- **Admin Profile Creation**: Allows the admin to create a user profile if it doesn't already exist.
- **Add User Profile**: Enables the addition of user profiles by their user ID.
- **Assign Tokens**: Allows the admin to assign tokens to a user, checking for sufficient balance.
- **Minter**: Allows the admin to mint a specified number of tokens.
- **Transfer Tokens**: Enables the transfer of tokens from one user to another.
- **Burn Tokens**: Allows a user to burn their tokens, reducing their balance.

## Functions

### 1. `admin_profile(user: User)`

- **Description**: Creates a user profile for the admin if it does not already exist.
- **Parameters**: 
  - `user`: A `User` object containing user details.
- **Panic**: If the admin profile already exists.

### 2. `add_user_profile(user_id: String, user: User)`

- **Description**: Adds a new user profile.
- **Parameters**:
  - `user_id`: The unique identifier for the user.
  - `user`: A `User` object containing user details.
- **Panic**: If the user profile with the given `user_id` already exists.

### 3. `assign_tokens(user_id: String, amount: i32)`

- **Description**: Assigns tokens from the admin to a specified user.
- **Parameters**:
  - `user_id`: The ID of the user receiving tokens.
  - `amount`: The number of tokens to assign.
- **Panic**: If the admin does not have enough tokens to assign.

### 4. `minter()`

- **Description**: Mints a specified number of tokens for the admin.
- **Parameters**: None.
- **Notes**: Currently, it mints a fixed amount of 100,000 tokens.

### 5. `transfer_tokens(sender: String, receiver: String, number_tokens: i32)`

- **Description**: Transfers tokens from one user to another.
- **Parameters**:
  - `sender`: The ID of the sender.
  - `receiver`: The ID of the receiver.
  - `number_tokens`: The number of tokens to transfer.
- **Panic**: If the sender does not have enough tokens to send.

### 6. `burn_tokens(burner_id: String, number_tokens: i32)`

- **Description**: Allows a user to burn their tokens.
- **Parameters**:
  - `burner_id`: The ID of the user burning tokens.
  - `number_tokens`: The number of tokens to burn.
- **Panic**: If the user does not have enough tokens to burn.

### 7. `get_user_profile(user_id: String) -> Option<User>`

- **Description**: Retrieves the user profile for a given user ID.
- **Parameters**:
  - `user_id`: The unique identifier for the user.
- **Returns**: An `Option<User>` containing the user profile or `None` if it does not exist.

## Getting Started

To get started with the token manipulation smart contract:

1. Clone the repository.
2. Set up your development environment for Rust and the Internet Computer SDK.
3. Compile the smart contract and deploy it to the Internet Computer.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- Thanks to the DFINITY team for their support and documentation.
- Special thanks to the Rust community for their resources and libraries.
