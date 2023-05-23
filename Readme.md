# The `num!` and `hash!` macros for `ethers-core` types

Within the  macros arguments, you can write `U256`, `I256`, `U128`, `H256` and `H160` literals using the [same syntax][rust-syntax] as Rust integer literals, but using a capital `U`, `I` or `H` suffix respectively.
In order to make it works, you need to import `ethers`. 

I just readapted https://github.com/recmo/uint

[rust-syntax]: https://doc.rust-lang.org/stable/reference/tokens.html#integer-literals

## Examples
```rust
use ethers_literal::{num, hash};
let a = num!(4_U128);
const b: U256 = num!(42_U256);
const c: I256 = num!(-0xa3_I256);

let addr1 = hash!(0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640_H160);
const hash: H256 = hash!(0x4000000000000000000000000040000000000000000000000000000000000000_H256);
```
