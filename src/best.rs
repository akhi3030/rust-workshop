// Even better!

struct Balance(u64);
struct Amount(u64);

fn withdraw_funds_best(balance: Balance, amount_to_withdraw: Amount) -> Result<Balance, String> {
    if amount_to_withdraw > balance {
        return Err("not enough funds".to_string());
    }
    Ok(balance - amount_to_withdraw)
}
