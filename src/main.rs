// What can be improved in the following code?
fn withdraw_funds(balance: u64, amount_to_withdraw: u64) -> Result<u64, String> {
    if amount_to_withdraw > balance {
        return Err("not enough funds".to_string());
    }
    Ok(balance - amount_to_withdraw)
}

// Let's make the code more readable

/*
 type Balance = u64;
type Amount = u64;

fn withdraw_funds_better(balance: Balance, amount_to_withdraw: Amount) -> Result<Balance, String> {
    if amount_to_withdraw > balance {
        return Err("not enough funds".to_string());
    }
    Ok(balance - amount_to_withdraw)
}

*/

// Even better!

pub struct Balance(u64);
pub struct Amount(u64);

impl Balance {
    fn get(&self) -> u64 {
        self.0
    }
}

impl Amount {
    fn get(&self) -> u64 {
        self.0
    }
}

fn withdraw_funds_even_better(
    balance: Balance,
    amount_to_withdraw: Amount,
) -> Result<Balance, String> {
    if amount_to_withdraw.get() > balance.get() {
        return Err("not enough funds".to_string());
    }
    Ok(Balance(balance.get() - amount_to_withdraw.get()))
}

pub struct UpdateableConfigs {
    /// `None` means that the validator key existence could not be determined.
    /// `Some(None)` means that it was determined that the validator key does not exist.
    pub validator_signer: Option<UpdateableValidatorSigner>,
}

fn main() {}
