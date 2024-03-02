use super::*;

#[derive(Clone, Debug)]
pub struct TailwindClear {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindClear => "clear");

impl TailwindClear {
    /// <https://tailwindcss.com/docs/clear>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("clear", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/clear#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "both",
            "inherit",
            "initial",
            "inline-end",
            "inline-start",
            "left",
            "none",
            "revert",
            "right",
            "unset",
        ]);
        set.contains(mode)
    }
}
