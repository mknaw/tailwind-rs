use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindBackgroundAttachment => "background-attachment");

impl Display for TailwindBackgroundAttachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-")?;
        let s = self.kind.get_properties();
        match s {
            s @ ("fixed" | "local" | "scroll") => write!(f, "{}", s),
            _ => write!(f, "attach-{}", s),
        }
    }
}

impl TailwindBackgroundAttachment {
    /// <https://tailwindcss.com/docs/background-attachment>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("bg-attach", &check_valid)(pattern, arbitrary)? })
    }
    /// <https://tailwindcss.com/docs/background-attachment>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment#syntax>
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        // Keyword values
        "auto",
        "avoid",
        // Page break values
        "avoid-page",
        // Column break values
        "avoid-column",
        // Region break values
        "avoid-region",
        // Global values
        "inherit",
        "initial",
        "revert",
        "unset",
    ]);
    set.contains(mode)
}
