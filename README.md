# ICP Token Wallet

This project implements a simple token wallet smart contract for the Internet Computer Protocol (ICP) using Rust. The wallet allows users to send and receive tokens, as well as check their balance.

## Evaluation Criteria

1. **Functionality**: The wallet performs all specified operations without errors:
   - Send tokens
   - Receive tokens
   - Display token balance

2. **Code Quality**: The code is clean, well-organized, and thoroughly commented.

3. **Security**: Basic security measures are implemented to protect wallet transactions:
   - Balance checks before sending tokens
   - Use of `ic_cdk::caller()` to identify the transaction initiator

4. **Documentation**: Clear and comprehensive documentation for setting up and operating the wallet (this README).

## Setup

1. Ensure you have the following installed:
   - Rust (https://www.rust-lang.org/tools/install)
   - dfx (https://internetcomputer.org/docs/current/developer-tools/deploying/install-upgrade-remove)

2. Clone this repository:
   ```
   git clone https://github.com/your-username/icp-token-wallet.git
   cd icp_token_wallet
   ```

3. Install dependencies and generate the lockfile:
   ```
   cargo build
   cargo generate-lockfile
   ```

## Building and Deploying

1. Start the local Internet Computer network:
   ```
   dfx start --clean --background
   ```

2. Deploy the canister:
   ```
   dfx deploy
   ```

## Usage

After deployment, interact with the smart contract using the `dfx` command-line tool:

1. Get your balance:
   ```
   dfx canister call icp_token_wallet get_balance
   ```

2. Receive tokens (for testing):
   ```
   dfx canister call icp_token_wallet receive_tokens '(100)'
   ```

3. Send tokens:
   ```
   dfx canister call icp_token_wallet send_tokens '(record { to = "recipient-principal-id"; amount = 50 })'
   ```
   Replace "recipient-principal-id" with the actual principal ID of the recipient.

## Testing

Run the unit tests:

```
cargo test
```

## Code Structure

- `src/lib.rs`: Contains the main smart contract logic.
- `src/icp_token_wallet.did`: Candid interface file for the smart contract.
- `Cargo.toml`: Rust package configuration file.
- `dfx.json`: DFX configuration file for deploying the canister.

## Security Considerations

- The wallet uses `ic_cdk::caller()` to identify the transaction initiator, ensuring that only the owner can send their tokens.
- Balance checks are performed before sending tokens to prevent overdrafts.
- For a production environment, additional security measures should be implemented, such as:
  - Multi-signature transactions
  - Rate limiting
  - Enhanced error handling and logging

## Future Improvements

- Implement more advanced features like token minting and burning.
- Add a frontend interface for easier interaction with the wallet.
- Enhance security with multi-signature support and more robust error handling.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.