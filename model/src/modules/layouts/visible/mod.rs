use super::*;

#[derive(Debug, Clone)]
pub struct TailwindVisibility {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindVisibility => "visibility");

impl TailwindVisibility {
    /// <https://tailwindcss.com/docs/visibility>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => StandardValue::from("visible"),
            ["none"] => StandardValue::from("hidden"),
            _ => StandardValue::parser("visible", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/visibility#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "collapse", "hidden", "inherit", "initial", "revert", "unset", "visible",
        ]);
        set.contains(mode)
    }
}
