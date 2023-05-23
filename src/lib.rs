use proc_macro::TokenStream;

mod hash;
mod num;

use hash::transform_stream_hash;
use num::transform_stream_num;

// Repeat the crate doc
#[doc = include_str!("../Readme.md")]
#[proc_macro]
pub fn num(stream: TokenStream) -> TokenStream {
    transform_stream_num(stream)
}

// Repeat the crate doc
#[doc = include_str!("../Readme.md")]
#[proc_macro]
pub fn hash(stream: TokenStream) -> TokenStream {
    transform_stream_hash(stream)
}
