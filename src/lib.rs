pub mod nibbles;
pub mod node;
mod tests;

mod db;
mod errors;
mod trie;

pub use db::{MemoryDB, DB};
pub use errors::{MemDBError, TrieError};
pub use trie::{decode_node, EthTrie, RootWithTrieDiff, Trie};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
