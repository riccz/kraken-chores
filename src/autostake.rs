use kraken_rest_client::{api::ExtendedBalance, Client};
use rust_decimal::Decimal;

use anyhow::Result;

#[derive(Debug, clap::Args, Clone)]
pub struct Args {}

fn available_balance(ext_bal: &ExtendedBalance, with_credit: bool) -> Result<Decimal> {
    let balance: Decimal = ext_bal.balance.parse()?;
    let hold_trade: Decimal = ext_bal.hold_trade.parse()?;
    let mut avail = balance - hold_trade;

    if with_credit {
        let credit: Decimal = ext_bal
            .credit
            .as_ref()
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(Decimal::ZERO);
        let credit_used: Decimal = ext_bal
            .credit_used
            .as_ref()
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(Decimal::ZERO);
        avail += credit - credit_used;
    }

    Ok(avail)
}

pub async fn run(client: &Client, args: Args) -> Result<()> {
    let balances = client.get_extended_balance().send().await?;

    let eth_balance = balances
        .get("XETH")
        .ok_or(anyhow!("XETH balance missing"))?;
    let avail_eth = available_balance(eth_balance, false)?;

    println!("Avail XETH balance: {}", avail_eth);

    Ok(())
}
