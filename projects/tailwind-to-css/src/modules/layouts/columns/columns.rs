use super::*;

#[derive(Clone, Debug)]
pub enum Columns {
    Columns(u8),
    Length(LengthUnit),
    Standard(String),
}

impl Display for Columns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Columns(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(g) => write!(f, "{}", g),
        }
    }
}

impl Columns {
    #[inline]
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |n: usize| Self::Length(LengthUnit::rem(n as f32));
        let out = match input {
            ["auto"] => Self::Standard("auto".to_string()),
            ["3xs"] => rem(16),
            ["2xs"] => rem(18),
            ["xs"] => rem(20),
            ["sm"] => rem(24),
            ["md"] => rem(28),
            ["lg"] => rem(32),
            ["xl"] => rem(36),
            ["2xl"] => rem(42),
            ["3xl"] => rem(48),
            ["4xl"] => rem(56),
            ["5xl"] => rem(64),
            ["6xl"] => rem(72),
            ["7xl"] => rem(80),
            [name] => {
                debug_assert!(!name.contains('%'), "forbidden use percent");
                let a = TailwindArbitrary::from(*name);
                let maybe_unit = || -> Result<Self> { Ok(Self::Columns(a.as_integer()?)) };
                let maybe_length = || -> Result<Self> { Ok(Self::Length(a.as_length()?)) };
                maybe_unit().or_else(|_| maybe_length())?
            },
            [] => Self::Length(arbitrary.as_length()?),
            _ => return syntax_error!("Unknown column instructions: {}", input.join("-")),
        };
        Ok(out)
    }
}
