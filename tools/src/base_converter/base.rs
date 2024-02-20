#[derive(Clone, Copy, derive_more::Display)]
pub enum Base {
    #[display(fmt = "2")]
    BINARY,
    #[display(fmt = "10")]
    DECIMAL,
    #[display(fmt = "16")]
    HEXADECIMAL,
}

impl std::convert::Into<u32> for Base {
    fn into(self) -> u32 {
        match self {
            Base::BINARY => 2,
            Base::DECIMAL => 10,
            Base::HEXADECIMAL => 16,
        }
    }
}

impl std::convert::TryFrom<&str> for Base {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // (ref.) [How to match a String against string literals?](https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals)
        match value {
            "2" => Ok(Base::BINARY),
            "10" => Ok(Base::DECIMAL),
            "16" => Ok(Base::HEXADECIMAL),
            _ => anyhow::bail!("invalid base!"),
        }
    }
}
