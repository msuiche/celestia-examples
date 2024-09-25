// Lumina RPC Client
// https://github.com/eigerco/lumina/blob/main/rpc/

// Step 1: Create keys
// ./cel-key add my_celes_key --keyring-backend test --node.type light --p2p.network celestia

// Step 2: Get the auth token
// ./cel-key list --node.type light --keyring-backend test --p2p.network celestia

// Step 3: Start the light node
// celestia light start --keyring.keyname my_celes_key --core.ip consensus.lunaroasis.net

// Step 4: Run the Rust client
// cargo run -- --celestia-node-url http://localhost:26658 --celestia-node-auth-token <your_auth_token>

use clap::Parser;

use celestia_rpc::{BlobClient, HeaderClient, Client};
use celestia_types::{nmt::Namespace, Blob, blob::SubmitOptions};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL for the Celestia node
    // The default URL is http://localhost:26658.
    // For subscription methods (e.g., SubscribeHeaders), use the WebSocket protocol:
    // ws://localhost:26658
    #[arg(short = 'u', long, env = "CELESTIA_NODE_URL", default_value = "ws://localhost:26658")]
    celestia_node_url: String,

    /// Authentication token for the Celestia node
    // To get your auth token, see this guide:
    // https://docs.celestia.org/developers/node-tutorial#auth-token
    // To run your node without an auth token, use the --rpc.skip-auth flag when starting your node.
    // This allows you to pass an empty string as your auth token.
    // Client::new() only accepts "None" or "Some(String)". No empty string is allowed.
    #[arg(short = 't', long, env = "CELESTIA_NODE_AUTH_TOKEN")]
    celestia_node_auth_token: Option<String>,
}

#[tokio::main]
async fn main() {
    let url = Args::parse().celestia_node_url;
    let token = Args::parse().celestia_node_auth_token;

    println!("URL: {}", url);
    println!("Token: {:?}", token);

    if token.is_none() {
        println!("WARNING: The authentication token is not provided. Make sure the light node is running with --rpc.skip-auth");
        println!("  i.e. `celestia light start --core.ip rpc.celestia.pops.one --p2p.network celestia --rpc.skip-auth`");
        println!("  If you are running a full node, you can set the token with `export CELESTIA_NODE_AUTH_TOKEN=<token>`");
        println!("");
    }

    subscribe_headers(&url, token.as_deref()).await;
    // submit_blob(&url, &token).await;
}

// To use the following methods, you will need the node URL and your auth token.

// Submit a blob to the network
// This is a simple example that creates a blob, submits it to the network, and then retrieves it.
// See the following for more information:
// https://github.com/eigerco/lumina/blob/main/rpc/tests/utils/client.rs
async fn submit_blob(url: &str, token: Option<&str>) {
    let client = Client::new(url, token)
        .await
        .expect("Failed creating rpc client");

    // let's use the DEADBEEF namespace
    let namespace = Namespace::new_v0(&[0xDE, 0xAD, 0xBE, 0xEF]).expect("Invalid namespace");

    // create a blob
    let blob = Blob::new(namespace, b"Hello, World!".to_vec()).expect("Blob creation failed");

    // submit the blob to the network
    let height = client
        .blob_submit(&[blob.clone()], SubmitOptions::default())
        .await
        .expect("Failed submitting blob");

    println!("Blob was included at height {}", height);

    // fetch the blob back from the network
    let retrieved_blobs = client
        .blob_get_all(height, &[namespace])
        .await
        .expect("Failed to retrieve blobs");

    assert_eq!(retrieved_blobs.len(), 1);
    assert_eq!(retrieved_blobs[0].data, b"Hello, World!");
    assert_eq!(retrieved_blobs[0].commitment, blob.commitment);
}

async fn subscribe_headers(url: &str, token: Option<&str>) {   
    let client = Client::new(url, token)
        .await
        .expect("ERROR:Failed creating rpc client");

    let mut header_sub = client
        .header_subscribe()
        .await
        .expect("ERROR: Failed subscribing to incoming headers");

    // setup the namespace we will filter blobs by
    let namespace = Namespace::new_v0(&[0xDE, 0xAD, 0xBE, 0xEF]).expect("ERROR: Invalid namespace");

    while let Some(extended_header) = header_sub.next().await {
        match extended_header {
            Ok(header) => {
                println!("Header: {}", header);
                let height = header.header.height.value();
                // fetch all blobs at the height of the new header

                let blobs = match client.blob_get_all(height, &[namespace]).await {
                    Ok(blobs) => blobs,
                    Err(e) => {
                        eprintln!("ERROR: Error fetching blobs: {}", e);
                        continue;
                    }
                };

                println!(
                    "Found {} blobs at height {} in the 0xDEADBEEF namespace",
                    blobs.len(),
                    height
                );
            }
            Err(e) => {
                eprintln!("ERROR: Error receiving header: {}", e);
            }
        }
    }
}
