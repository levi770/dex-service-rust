## DEX Trading API Microservice (work in progress)

### Overview:
The DEX Trading API microservice is a robust and efficient system developed primarily in Rust, aimed to provide seamless integration capabilities with decentralized exchanges, especially catering to Uniswap v2 and v3. By leveraging the power and safety of Rust, combined with the capabilities of web3 and gRPC, this service ensures high-performance interactions with blockchain networks. Furthermore, with integrated wallet abstractions, it simplifies and abstracts wallet-related complexities for end-users and developers alike.

### Key Features:

1. **Rust-based Performance**: Built with the speed and memory safety of Rust, ensuring fast execution and minimal runtime errors.
2. **Web3 Integration**: Direct integration with Ethereum-based decentralized applications and services through the web3 protocol, enabling real-time blockchain interactions.
3. **gRPC Support**: Uses gRPC for efficient, language-agnostic, and low-latency communication, making it ideal for microservices architectures and remote procedure calls.
4. **Uniswap Compatibility**: Tailored to work seamlessly with both Uniswap v2 and v3, thus providing a vast pool of liquidity and trading pairs.
5. **Wallet Abstractions**: Facilitates easier integration and interaction with various wallet types, reducing the friction for users to trade and manage assets.

### Use Cases:

1. **Automated Trading Bots**: Provide bots with an efficient means to execute trades on DEX platforms, taking advantage of arbitrage opportunities or implementing specific trading strategies.
2. **Wallet Applications**: Enable wallet developers to integrate DEX trading functionalities directly into their applications, offering users a one-stop solution.
3. **DApp Integration**: Allow decentralized applications to integrate DEX trading capabilities, enhancing the overall user experience and utility of the application.

### System Requirements:

- OS: Linux/Windows/MacOS
- Runtime: Rust 1.5X or newer
- Dependencies: web3 1.X, gRPC latest version
- Network: Stable internet connection with access to Ethereum nodes

### Installation & Setup:

Ensure you have the following installed on your system:
- Rust (edition 2021 or newer)
- Cargo
- PostgreSQL (if it's not installed, refer to the PostgreSQL official [documentation](https://www.postgresql.org/download/) for installation guidelines)

### Step-by-Step Guide:

1. **Clone & Navigate**:
   ```
   git clone <repository_url>
   cd core-server-web3
   ```

2. **Environment Setup**:
   Use the `dotenv` crate to manage environment variables. Create a `.env` file in the root of the project with your database configurations:
   ```
   DATABASE_URL=postgres://<username>:<password>@<hostname>:<port>/<database_name>
   ```

3. **Dependencies Installation**:
   ```
   cargo build
   ```

4. **Setting Up Diesel ORM**:
   First, ensure the `diesel_cli` tool is installed. If not:
   ```
   cargo install diesel_cli --no-default-features --features postgres
   ```
   Initialize the Diesel setup:
   ```
   diesel setup
   ```

5. **Migration Handling**:
   - To create a new migration:
     ```
     diesel migration generate <migration_name>
     ```
   - To run all pending migrations:
     ```
     diesel migration run
     ```
   - To undo the last migration:
     ```
     diesel migration revert
     ```

6. **gRPC Protobuf Interfaces Setup**:
   Create a directory for your protobuf (`.proto`) files, e.g., `protos/`.
   
   For each `.proto` file, ensure you define your services and RPC methods.

   In the `build.rs` file at the root of your project, add:
   ```rust
   fn main() -> Result<(), Box<dyn std::error::Error>> {
       tonic_build::configure()
           .compile(&["path/to/your/proto/file.proto"], &["path/to/your/proto/directory/"])?;
       Ok(())
   }
   ```
   The above will generate Rust code from your `.proto` files when building the project.

7. **Run the Microservice**:
   ```
   cargo run
   ```

8. **(Optional) TLS for gRPC**:
   The `tonic` crate in this project is set up with the `tls` feature, which means it supports encrypted communication. If you plan to use TLS:
   - Generate or obtain TLS certificates for your server.
   - Modify your tonic server configuration to utilize the certificates.

### Endpoints:

1. **Trade**: Execute trades on Uniswap v2/v3.
2. **Fetch Liquidity**: Get liquidity data for a specific trading pair.
3. **Wallet Balance**: Check the balance of a connected wallet for a specific token.
4. **Add/Remove Liquidity**: Facilitate liquidity provision or removal from a specific pool.

### Account Service:

1. **List All Accounts**: 
   - Endpoint: `list`
   - Request: `EmptyRequest`
   - Response: `FindAllAccountsResponse`
     - Contains an array of `FindOneAccountResponse` which has `id`, `user_id`, and `address`.

2. **Find Account by ID**:
   - Endpoint: `byId`
   - Request: `ByIdRequest`
     - Requires an `id` parameter.
   - Response: `FindOneAccountResponse`
     - Contains `id`, `user_id`, and `address`.

3. **Find Account by User ID**:
   - Endpoint: `byUserId`
   - Request: `ByUserIdRequest`
     - Requires a `user_id` parameter.
   - Response: `FindOneAccountResponse`
     - Contains `id`, `user_id`, and `address`.

4. **Create an Account**:
   - Endpoint: `create`
   - Request: `CreateAccountRequest`
     - Requires a `user_id` parameter.
   - Response: `CreateAccountResponse`
     - Returns `id` and `address` of the newly created account.

### Trade Service:

1. **Execute a Swap**:
   - Endpoint: `swap`
   - Request: `SwapRequest`
     - Requires `user_id`, `chain_id`, `exchange`, `token0`, `token1`, `amount`, `slippage`, and `deadline`.
   - Response: `SwapResponse`
     - Returns the `hash` of the completed trade.

These endpoints allow interactions with accounts and trade-related functionalities, ensuring a comprehensive interface to handle user requests. As with any gRPC service, the communication will be strongly typed, and clients must use the respective message types when interacting with these endpoints.

### Conclusion:
The DEX Trading API microservice is an essential tool for any developer or organization looking to seamlessly integrate and leverage the power of decentralized exchanges. With its high-performance capabilities and user-friendly design, it promises to revolutionize the way we interact with DEX platforms.





