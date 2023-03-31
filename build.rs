fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["./src/proto/trade.proto", "./src/proto/account.proto"],
        &["proto"],
    )?;
    Ok(())
}
