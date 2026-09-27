//! On-disk snapshot format for Phase 1 MiniDB.
//!
//! # File layout
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │ Header (16 bytes, fixed size)                                │
//! ├──────────────┬───────────────┬───────────────────────────────┤
//! │ magic [4]    │ version u32   │ entry_count u64               │
//! │ b"RDB1"      │ little-endian │ little-endian                 │
//! └──────────────┴───────────────┴───────────────────────────────┘
//! ┌──────────────────────────────────────────────────────────────┐
//! │ Entry 0                                                      │
//! ├──────────────┬───────────────────────────────────────────────┤
//! │ key_len u32  │ key bytes [key_len]                           │
//! ├──────────────┼───────────────────────────────────────────────┤
//! │ value_len u32│ value bytes [value_len]                       │
//! └──────────────┴───────────────────────────────────────────────┘
//! ... repeated entry_count times, keys written in sorted order ...
//! ```
//!
//! # Rules
//!
//! - Only live key/value pairs are persisted; tombstones are omitted.
//! - All integers are little-endian.
//! - Keys in the file must be sorted (matches `BTreeMap` iteration order).
//! - Empty databases write a valid header with `entry_count = 0`.
//!
//! # Crash-safe write protocol (implemented in a later step)
//!
//! 1. Write full snapshot to `{path}.tmp`
//! 2. `fsync` the temp file
//! 3. Atomically rename `{path}.tmp` → `{path}`

/// Default snapshot filename inside a database directory.
pub const DATA_FILE_NAME: &str = "data.db";

/// Temporary file used during atomic snapshot writes.
pub const DATA_FILE_TMP_SUFFIX: &str = ".tmp";

/// Four-byte file magic identifying a Rustone Phase 1 snapshot.
pub const MAGIC: [u8; 4] = *b"RDB1";

/// Snapshot format version.
pub const VERSION: u32 = 1;

/// Header size in bytes: magic(4) + version(4) + entry_count(8).
pub const HEADER_SIZE: usize = 16;
