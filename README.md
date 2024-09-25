# Celestia Node Client API Examples

This repository contains examples of how to use the Celestia Node Client API in Rust. These examples demonstrate various operations such as submitting blobs, subscribing to headers, and interacting with the Celestia network.

## Prerequisites

Before running these examples, make sure you have:

1. Rust and Cargo installed
2. A running Celestia node (light or full)
3. The necessary Rust dependencies installed (see `Cargo.toml`)

## Getting Started

### 1. Create keys: (optional)
1. Create keys:
   ```
   ./cel-key add my_celes_key --keyring-backend test --node.type light --p2p.network celestia
   ```

2. Get the auth token:
   ```
   ./cel-key list --node.type light --keyring-backend test --p2p.network celestia
   ```

### 2. Start the light node:

3. Start the light node:
   ```
   celestia light start --keyring.keyname my_celes_key --core.ip consensus.lunaroasis.net
   ```

If you opted out of the key creation, you can use the following command to start the node without a key:
   ```
   celestia light start --core.ip consensus.lunaroasis.net --rpc.skip-auth
   ```

## Examples

### 1. Submitting a Blob

This example demonstrates how to create a blob, submit it to the network, and then retrieve it.

### 2. Subscribing to Headers

This example shows how to subscribe to incoming headers and filter blobs by a specific namespace.

No key is needed for this example.


### Running the Examples

To run the Rust client:

```
cargo run -- --celestia-node-url http://localhost:26658 --celestia-node-auth-token <your_auth_token>
```

Note: If you're running your node with `--rpc.skip-auth`, you can omit the auth token.

## Important Notes

- The default URL for the Celestia node is `http://localhost:26658`. For subscription methods (e.g., SubscribeHeaders), use the WebSocket protocol: `ws://localhost:26658`.
- Make sure to replace `<your_auth_token>` with your actual authentication token.
- If running a node without an auth token, use the `--rpc.skip-auth` flag when starting your node.

## Troubleshooting

- If you encounter authentication issues, ensure that your auth token is correct and that your node is running with the appropriate settings.
- For WebSocket connections, make sure your firewall allows WebSocket traffic on the specified port.

## Further Reading

For more detailed information about the Celestia types and available methods, refer to the Rust documentation and the Celestia documentation:

- [Celestia Documentation](https://docs.celestia.org/)
- [Lumina RPC Client Repository](https://github.com/eigerco/lumina/blob/main/rpc/)
