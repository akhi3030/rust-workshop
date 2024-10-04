# Using better types

Is the following code good enough?

```rust
fn withdraw_funds(balance: u64, amount_to_withdraw: u64) -> Result<u64, String> {
    if amount_to_withdraw > balance {
        return Err("not enough funds".to_string());
    }
    Ok(balance - amount_to_withdraw)
}
```

Let's make the code more readable

```rust
 type Balance = u64;
type Amount = u64;

fn withdraw_funds_better(balance: Balance, amount_to_withdraw: Amount) -> Result<Balance, String> {
    if amount_to_withdraw > balance {
        return Err("not enough funds".to_string());
    }
    Ok(balance - amount_to_withdraw)
}
```

We can still make it better

```rust
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
```

# What are type aliases actually good for?

Absolutely nothing!  Just kidding =)

Use them to make it easier to read and write large types e.g. `Result<HashMap<FlatStorageDelta, FlatStorageValue>, FlatStorageLookupError>` could be type aliases to `FlatStorageLookupResult`.

* We have to read and write less code and many types we may not really care what exactly the precise type is.
* Chances are small that such a precise type exists elsewhere in the code so that we mistake it for the wrong argument.

# Use the type system to help document code

```rust
struct UpdateableConfigs {
    /// `None` means that the validator key existence could not be determined.
    /// `Some(None)` means that it was determined that the validator key does not exist.
    pub validator_signer: Option<Option<Arc<ValidatorSigner>>>,
}
```

The problem with the above code is that when I see that `validator_signer` is set to `None` or `Some(None)` at some random point in the code base, I need to look up the definitions to figure out what that actually means.

```rust

enum ValidatorSignerKey {
    KeyExistenceNotDetermined,
    KeyDoesNotExist,
    Key(ValidatorSigner),
}

struct UpdateableConfigs {
    pub validator_signer: ValidatorSignerKey,
}
```

Actual names of the variants can be debated but in general this makes the code a bit more legible.

# Don't use `_` when pattern matching

```rust

enum PeerStatus {
    /// Handshake in progress.
    Connecting(Url),
    /// Ready to go.
    Ready(Connection),
}

fn handle_peer(status: PeerStatus) {
    match status {
        PeerStatus::Connecting(url) => handle_connecting_peer(url),
        _ => panic!("Connection should be in connecting status"),
    }
}
```

The problem is that later when you want to add a new variant to `PeerStatus`, the compiler will not help you find all the locations where the enum is being used and might need updating.  
