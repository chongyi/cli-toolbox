use clap::{Args, Subcommand};

use self::uuid::UuidToolkit;

pub mod uuid;

#[derive(Subcommand)]
pub enum Toolkit {
    Uuid(UuidToolkit)
}