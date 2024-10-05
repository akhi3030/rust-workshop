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
    // Note that this function is not `pub`.  This helps with additional safety but might be too cumbersome.
    fn get(&self) -> u64 {
        self.0
    }
}

impl Amount {
    // Note that this function is not `pub`.  This helps with additional safety but might be too cumbersome.
    fn get(&self) -> u64 {
        self.0
    }
}

pub fn withdraw_funds_even_better(
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

Use them to make it easier to read and write large types e.g. `Result<HashMap<FlatStorageDelta, FlatStorageValue>, FlatStorageLookupError>` could be type aliased to `FlatStorageLookupResult`.

* We have to read and write less code
* We may not really care what exactly the precise type is when we are reading code.  By looking at alias, we can guess that it is a `Result<>` type and make some assumptions about what is being returned in the two variants.
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

# Parse don't validate example

```rust
fn validate_no_dup_shard_ids(shard_ids: Vec<ShardId>) {
    if dups_found {
        panic!();
    }
}

fn count_accounts_in_shards(shard_ids: Vec<ShardId>) -> usize {
    assert!(validate_no_dup_shard_ids(shard_ids));
    // ...
}
```

In the above example, we might forget to call `validate_no_dup_shard_ids()`.

```rust
fn validate_no_dup_shard_ids(shard_ids: Vec<ShardId>) -> HashSet<ShardId> {
    if dups_found {
        panic!();
    }
}

fn count_accounts_in_shards(shard_ids: HashSet<ShardId>) -> usize {
    // ...
}

```

If we redefine `count_accounts_in_shards()` to only accept valid input, then it is "guaranteed" not to panic and we do not have to worry about checking if the caller has validated the input or we need to validate, etc.

# Boolean blindness

Lots of articles on the Internet.  I couldn't find one that I really liked.  The basic idea is that boolean values do not provide enough expressiveness.

```rust
fn set_loading_status(enable: bool) {...}

set_loading_status(true)
set_loading_status(false)
```

When you look at the call sites, it is not very easy to tell what exactly is being done.  Instead consider either of the following options.

```rust
fn enable_loading_status() {...}
fn disable_loading_status() {...}
```

or 

```rust
enum LoadingStatus{
    Enable,
    Disable,
}

fn set_loading_status(status: LoadingStatus) {...}

set_loading_status(LoadingStatus::Enable);
```

Both of the options above are much easier to read and an improvement over the original.

# Do not assume that you will always be using an IDE

A IDE provides a lot of support.  For inferred types, it will tell us what the precise type is; by hovering over variables, etc. we can quickly look up the type definitions.

However, there are many occassions when we have to look at code and loading it in an IDE would not be possible or be too cumbersome.  Some examples:

* We are looking at some code on a production machine or on github or other places where the IDE is not installed.
* We are looking at code at different commits (e.g. using `git cat`) and it would be too cumbersome to actually checkout code at those commit points and wait for the analyzer to finish running.
