use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDelay {
    ms: NumericValue,
}

impl TailwindInstance for TailwindDelay {
    fn collision_id(&self) -> &'static str {
        "transition-delay"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindDelay {
    /// <https://tailwindcss.com/docs/transition-delay>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let ms = match pattern {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("delay", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { ms })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-delay#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
