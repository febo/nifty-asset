use super::*;

pub struct BurnArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub recipient: Option<Pubkey>,
}

pub fn handle_burn(args: BurnArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;

    let signer = signer_sk.pubkey();
    let asset = args.asset;

    let ix = Burn {
        asset,
        signer,
        recipient: args.recipient,
    }
    .instruction();

    let sig = send_and_confirm_tx(&config.client, &[&signer_sk], &[ix])?;

    println!("Burned asset {asset} in tx: {sig}");

    Ok(())
}
