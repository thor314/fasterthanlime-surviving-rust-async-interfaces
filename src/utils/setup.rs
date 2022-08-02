//! Setup utils for the crate
//! https://github.com/Keats/validator

use anyhow::Result;
use clap::Parser;
use validator::{Validate, ValidationError};

use super::cli::Args;

/// Set up crate context, cli, logging, and environment variables.
pub(crate) fn setup() -> Result<Context> {
  dotenv::dotenv().ok();
  init_tracing();
  let context = Context::new(Args::parse());
  context.validate()?;
  Ok(context)
}

/// A template struct to set up some context for main.
/// Use the excellent Validater library to validate the context before use.
#[derive(Clone, Debug, Validate)]
pub(crate) struct Context {
  /// args to clap CLI
  #[validate]
  pub args: Args,
  /// example context field
  #[validate(custom = "valid")]
  pub s:    String,
}

impl Context {
  #[tracing::instrument] // declare a tracing span to log
  fn new(args: Args) -> Self {
    tracing::info!("async setting up context");
    Self { s: "".into(), args }
  }
}

/// A template function to validate a Context argument
fn valid(_s: &str) -> Result<(), ValidationError> { Ok(()) }
/// Set up the tracing filter level using the env value, or else set it here. Reads RUST_LOG.
/// TRACE < DEBUG < INFO < WARN < ERROR
pub(crate) fn init_tracing() {
  let filter = tracing_subscriber::filter::LevelFilter::INFO.into();
  tracing_subscriber::filter::EnvFilter::builder().with_default_directive(filter).from_env_lossy();
}
