use anyhow::Result;
use clap::Clap;

use crate::traits::SubCommandVariant;

/// Simple calc, power by rust fasteval
#[derive(Clap)]
pub struct Calc {
    /// Calculation expression
    #[clap(name = "EXPR")]
    expr: String,
}

impl SubCommandVariant for Calc {
    fn execute(&self) -> Result<()> {
        let val = fasteval::ez_eval(&self.expr, &mut fasteval::EmptyNamespace)?;

        println!("{}", val);

        Ok(())
    }
}
