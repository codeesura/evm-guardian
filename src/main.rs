use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256, address},
    providers::{
        Provider, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    time::Duration,
};
use tokio::{task, time::interval};

const RPC_URL: &str = "https://mainnet.storyrpc.io";
const TARGET_ADDRESS: Address = address!("B09FF7F74e627Ac36F7Ddf2dBDBF9CBea9350Aa0"); // secyre wallet address
const CHECK_INTERVAL: u64 = 2; // 2 second rate limit tested best optimal limit

type ProviderType = FillProvider<
    JoinFill<
        alloy::providers::Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

struct WalletManager {
    signer: PrivateKeySigner,
    provider: Arc<ProviderType>,
}

impl WalletManager {
    async fn new(signer: PrivateKeySigner) -> Result<Self> {
        let provider = ProviderBuilder::new().on_http(RPC_URL.parse()?);
        Ok(Self {
            signer,
            provider: Arc::new(provider),
        })
    }

    async fn get_eth_balance(&self) -> Result<U256> {
        self.provider
            .get_balance(self.signer.address())
            .await
            .map_err(|e| eyre::eyre!("Balance query error: {}", e))
    }

    async fn transfer_eth(&self) -> Result<()> {
        let balance = self.get_eth_balance().await?;
        if balance < U256::from(10_000000000000000_u128) {
            return Ok(());
        }

        let estimate_tx = TransactionRequest::default()
            .with_to(TARGET_ADDRESS)
            .with_value(U256::from(100));

        let (gas_price, nonce, gas_limit) = tokio::try_join!(
            self.provider.get_gas_price(),
            self.provider.get_transaction_count(self.signer.address()),
            self.provider.estimate_gas(&estimate_tx),
        )?;

        let tx = TransactionRequest::default()
            .with_to(TARGET_ADDRESS)
            .with_value(balance - (U256::from(gas_limit) * U256::from(gas_price)))
            .with_nonce(nonce)
            .with_gas_price(gas_price)
            .with_gas_limit(gas_limit); // 21_000

        let envelope = tx.build(&EthereumWallet::from(self.signer.clone())).await?;
        let _ = self.provider.send_tx_envelope(envelope).await?;

        Ok(())
    }
}

async fn monitor_wallet(wallet: WalletManager) {
    let address = wallet.signer.address();
    let mut interval = interval(Duration::from_secs(CHECK_INTERVAL));

    loop {
        interval.tick().await;

        match wallet.transfer_eth().await {
            Ok(_) => println!("[{:?}] Success", address),
            Err(e) => println!("[{:?}] Error: {}", address, e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let private_keys = read_private_keys("private-keys.txt")?;

    let mut handles = vec![];
    for pk in private_keys {
        let wallet = WalletManager::new(pk).await?;
        handles.push(task::spawn(monitor_wallet(wallet)));
    }

    futures::future::join_all(handles).await;
    Ok(())
}

fn read_private_keys<P: AsRef<Path>>(path: P) -> Result<Vec<PrivateKeySigner>> {
    let file = File::open(path)?;
    io::BufReader::new(file)
        .lines()
        .map(|line| line?.trim().parse().map_err(Into::into))
        .collect()
}
