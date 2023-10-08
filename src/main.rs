use descent_relayer_client::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = run().await?;

    Ok(())
}
