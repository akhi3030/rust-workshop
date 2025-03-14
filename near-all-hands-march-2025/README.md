# Brief review

[Presentation](https://github.com/akhi3030/rust-workshop/blob/main/london-offsite-2024/README.md) I gave during the last offsite in London.

Very important section: [Parse, don't validate](https://github.com/akhi3030/rust-workshop/blob/main/README.md#parse-dont-validate) and [this](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) blog post.

# ValidatedTransaction

- We used to store transactions as `SignedTransaction`s everywhere in the code.  And we would either explicitly verify the signature or assume that the caller has verified the signature when needed.
- A couple of weeks ago, we [noticed](https://near.zulipchat.com/#narrow/channel/295302-general/topic/Unnecessary.20signature.20verification.3F) that we were doing verifying signatures multiple times unnecessarily.
  - In particular, we [verify signature](https://github.com/near/nearcore/blob/1ce9e630557fd871b0dd393262074abb347373c2/chain/client/src/client.rs#L2224) when we receive it from the RPC node and then put it in the transaction pool.
  - And then [again](https://github.com/near/nearcore/blob/1ce9e630557fd871b0dd393262074abb347373c2/chain/chain/src/runtime/mod.rs#L778) after we take the transaction from the transaction pool to prepare a chunk.
- Simplest fix would be a one line change, simply set `verify_signature` to `false` when we take the tx out of the tx pool.
- This change would not be easy to understand and risky.
- A better fix is to introduce `ValidatedTransaction` which can only be constructed by verifying signatures and performing other validations.  
- Tx pools store `ValidatedTransaction`s so any consumer of txs from the pool can be confident that the verification has happened and they do not need to repeat it.

## Benefits

- Can remove a duplicate signature check without losing confidence in correctness
- Greatly increased the readability of the code
- Allowed for cleaner abstractions which actually allowed us to [parallelise things](https://github.com/near/nearcore/pull/13081/files) better and improve performance even more.

# Strive for better interfaces

- So often, we have code that passes objects as reference and then clones it.  E.g. [here](https://github.com/near/nearcore/pull/13112/files).
- Similarly, we clone objects, pass it by value, and then ultimately, only need a reference to it and then drop it.  E.g. [here](https://github.com/near/nearcore/pull/13098/files)

**Our code has a lot of unnecessary complexity.**

E.g.: [here](https://github.com/near/nearcore/blob/master/chain/chain/src/store/mod.rs#L1828)
- This is quite a lot of unnecessary complexity.  Not only could we potentially store the `Block` as an `Option<Block>` but we could actually improve the overall [API](https://github.com/near/nearcore/blob/master/chain/chain/src/store/mod.rs#L1550) as we would expect `save_block` to be called at most once.

When we are adding new features, we need to be vigilent and fix this complexity along the way.  Unnecessary complexity:
- Makes it harder for new people to understand the code
- Makes it harder for you to add new features
- Makes it more likely that there are performance bugs (as we have already seen) and even worse that there are functional bugs
