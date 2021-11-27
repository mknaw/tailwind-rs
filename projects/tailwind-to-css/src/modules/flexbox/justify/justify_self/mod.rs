use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifySelf {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindJustifySelf => "justify-self");

impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-self-{}", self.kind)
    }
}

impl TailwindJustifySelf {
    /// <https://tailwindcss.com/docs/justify-self>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("justify-self", &check_valid)(pattern, arbitrary)? })
    }
    /// <https://tailwindcss.com/docs/justify-self>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self#syntax
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        "auto",
        "baseline",
        "center",
        "end",
        "flex-end",
        "flex-start",
        "inherit",
        "initial",
        "left",
        "normal",
        "revert",
        "right",
        "self-end",
        "self-start",
        "start",
        "stretch",
        "unset",
    ]);
    set.contains(mode)
}
