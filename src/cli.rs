use clap::Parser;

use crate::toolkit::{Toolkit, uuid::UuidToolkit};

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    toolkit: Toolkit
}

impl Args {
    pub fn run_toolkit(self) {
        match self.toolkit {
            Toolkit::Uuid(toolkit) => toolkit.run(),
        }
    }
}