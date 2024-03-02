use super::*;

#[derive(Clone, Debug)]
pub struct TailwindPosition {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindPosition => "position");

impl TailwindPosition {
    /// <https://tailwindcss.com/docs/position>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("position", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position#syntax>
    pub fn check_valid(mode: &str) -> bool {
        [
            "static", "relative", "absolute", "fixed", "sticky", "inherit", "initial", "revert",
            "unset",
        ]
        .contains(&mode)
    }
}
