//! Amnion is a read-only projection client for Pincher and Decapod state.
//!
//! The projection layer is deliberately independent from terminal rendering and
//! from any future Pincher transport. The fixture source is the only adapter in
//! this first slice; it is not a production protocol declaration.

pub mod app;
pub mod events;
pub mod fixtures;
pub mod projection;
pub mod sources;
pub mod ui;

pub use app::{App, Verbosity};
pub use events::{SourceEvent, SourceFreshness, SourceType};
pub use projection::{IntentProjection, Reducer};
