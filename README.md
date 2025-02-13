# EVM-Guardian

A high-performance, concurrent EVM-compatible wallet recovery tool built in Rust. This tool monitors specified wallets across multiple blockchain networks and automatically transfers funds to a secure destination address to protect assets from compromised wallets.

## Overview

This tool is designed to help recover funds from potentially compromised wallets on EVM-compatible networks by automatically monitoring balances and transferring funds to a secure destination address. It utilizes Rust's async capabilities and the Tokio runtime for efficient concurrent monitoring of multiple wallets across different networks.

### Key Features

- Concurrent monitoring of multiple wallets
- Support for all EVM-compatible networks
- Efficient gas price optimization
- Automatic balance sweeping
- Rate-limited monitoring (2 second intervals)
- High-performance Rust implementation
- Robust error handling and logging

## Prerequisites

- Rust (latest stable version)
- Access to EVM-compatible RPC endpoints
- Private keys of the wallets to monitor

## Installation

1. Clone the repository:

```
git clone https://github.com/codeesura/evm-guardian.git
cd evm-guardian
```

2. Build the project:

```
cargo build --release
```

## Configuration

1. Create a private-keys.txt file in the project root:
   - Add one private key per line
   - Each private key should be in hex format (without 0x prefix)

2. Configure the constants in src/main.rs:
   - RPC_URL: Your EVM network RPC endpoint (supports any EVM-compatible network)
   - TARGET_ADDRESS: The secure destination address
   - CHECK_INTERVAL: Monitoring interval in seconds (default: 2)

## Usage

1. Ensure your configuration is set up correctly
2. Run the tool:

cargo run --release

The tool will:
1. Load private keys from private-keys.txt
2. Monitor each wallet concurrently
3. Automatically transfer funds when balance exceeds 0.01 ETH
4. Log all operations to stdout

## Technical Details

### Architecture

- **WalletManager**: Handles individual wallet operations
  - Balance checking
  - Transaction building
  - ETH transfers
  
- **Concurrent Monitoring**: Uses Tokio for async execution
  - Each wallet runs in its own task
  - Non-blocking operations
  - Efficient resource utilization

### Transaction Logic

- Automatically calculates optimal gas prices
- Estimates gas limits for transactions
- Deducts gas costs from transfer amount
- Handles nonce management automatically

### Safety Features

- Rate limiting to prevent RPC overload
- Error handling and reporting
- Balance thresholds to prevent dust transfers
- Gas price optimization

## Dependencies

- alloy: Ethereum interaction library
- tokio: Async runtime
- eyre: Error handling
- futures: Async utilities

## Security Considerations

- Store private keys securely
- Use a reliable RPC endpoint
- Monitor the tool's operation
- Keep the destination address secure
- Regularly check logs for any issues

## Monitoring and Logging

The tool provides real-time logging:
- Success messages include wallet addresses
- Error messages with detailed information
- Transaction status updates

## Performance Optimization

The tool is optimized for:
- Minimal RPC calls
- Efficient memory usage
- Fast transaction processing
- Concurrent wallet monitoring

## Best Practices

1. Test with small amounts first
2. Use a reliable RPC endpoint
3. Monitor system resources
4. Keep private keys secure
5. Regular log checking

## Error Handling

The tool implements comprehensive error handling for:
- RPC connection issues
- Transaction failures
- Invalid private keys
- Network problems
- Gas estimation errors

## Contributing

Contributions are welcome! Please follow these steps:
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Create a pull request

## License

[MIT License](LICENSE)

## Disclaimer

This tool is provided for legitimate recovery operations only. Users are responsible for ensuring they have the right to access and transfer funds from the specified wallets.

## Support

For issues and support:
1. Open an issue in the repository
2. Provide detailed information about the problem
3. Include relevant logs and configuration

---

**Note**: Always verify addresses and configurations before running the tool with real funds.
