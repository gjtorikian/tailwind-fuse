use super::*;

#[derive(Debug, Clone)]
pub struct TailwindTorch {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindTorch => "user-select");

impl TailwindTorch {
    /// https://tailwindcss.com/docs/touch-action
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("torch", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "inherit",
            "initial",
            "manipulation",
            "none",
            "pan-down",
            "pan-left",
            "pan-right",
            "pan-up",
            "pan-x",
            "pan-y",
            "pinch-zoom",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
