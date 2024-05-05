use std::collections::HashMap;

use kraken_rest_client::{api::ExtendedBalance, order, Client, OrderSide};
use rust_decimal::Decimal;

use anyhow::Result;

use crate::common::available_balance;

#[derive(Debug, clap::Args, Clone)]
pub struct Args {}

fn split_by_weight(
    amount: Decimal,
    weights: &HashMap<String, Decimal>,
) -> HashMap<String, Decimal> {
    let s: Decimal = weights.iter().map(|(_, w)| w).sum();
    let mut out = HashMap::new();
    for (k, w) in weights {
        out.insert(k.to_string(), amount * w / s);
    }
    out
}

async fn get_prices(client: &Client, pairs: &Vec<String>) -> Result<HashMap<String, Decimal>> {
    let all_pairs = pairs.join(",");
    let tickers = client.get_tickers(&all_pairs).send().await?;
    let mut out = HashMap::new();
    for (pair, ticker) in tickers {
        // TODO: better price choice: middle of the book spread, checked against last 24h avg price?
        out.insert(pair, ticker.c[0].parse().unwrap());
    }
    Ok(out)
}

pub async fn run(client: &Client, args: Args, common_args: crate::CommonArgs) -> Result<()> {
    let balances = client.get_extended_balance().send().await?;
    // debug!("Extended balance: {balances:?}");

    // TODO: config to get this
    let base = "ZEUR";
    let mut weigths = HashMap::<String, Decimal>::new();
    weigths.insert("XXBT".to_string(), Decimal::from(90));
    weigths.insert("XETH".to_string(), Decimal::from(10));

    let avail_base = available_balance(
        balances.get(base).ok_or(anyhow!("Base balance missing"))?,
        false,
    )?;
    debug!("Available {base} balance: {avail_base}");

    let amounts_to_buy = split_by_weight(avail_base, &weigths);
    let pairs: Vec<String> = weigths.keys().map(|k| format!("{k}{base}")).collect();
    let prices = get_prices(client, &pairs).await?;

    for (k, amount) in amounts_to_buy {
        let pair = format!("{k}{base}");
        let price = prices.get(&pair).unwrap();

        debug!("Will buy {pair} at {price} for {amount} {base}");

        // TODO: abort if under min buy volume for this pair
        // TODO: better price selection
        // TODO: expire orders

        let vol = amount / price;
        let order_req = client
            .add_limit_order(&pair, OrderSide::Buy, &vol.to_string(), &price.to_string())
            .validate(common_args.dry_run)
            .send()
            .await?;

        info!(
            "Submitted order {:?}: {}",
            order_req.txid, order_req.descr.order
        );
    }

    Ok(())
}
