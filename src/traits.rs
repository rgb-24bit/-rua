use anyhow::Result;

pub trait SubCommandVariant {
    fn execute(&self) -> Result<()>;
}
