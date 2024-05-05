use anyhow::Result;
use kraken_rest_client::api::ExtendedBalance;
use rust_decimal::Decimal;

pub fn available_balance(ext_bal: &ExtendedBalance, with_credit: bool) -> Result<Decimal> {
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
