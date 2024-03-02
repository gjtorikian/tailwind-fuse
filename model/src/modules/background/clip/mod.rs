use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBackgroundClip => "background-clip");

impl TailwindBackgroundClip {
    /// <https://tailwindcss.com/docs/background-clip>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["border"] => StandardValue::from("border-box"),
            ["padding"] => StandardValue::from("padding-box"),
            ["content"] => StandardValue::from("content-box"),
            _ => StandardValue::parser("bg-clip", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "border-box",
            "content-box",
            "inherit",
            "initial",
            "padding-box",
            "revert",
            "text",
            "unset",
        ]);
        set.contains(mode)
    }
}
