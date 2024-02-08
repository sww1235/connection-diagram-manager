//! `cdm_traits` provide traits for Connection Diagram Manager
//!
//! They are in a separate crate due to issues with proc macro generation
//!

/// `merge` provides the `Merge` trait
pub mod merge;

/// `empty` provides the `Empty` trait
pub mod empty;

/// `partial_empty` provides the `PartialEmpty` trait
pub mod partial_empty;
