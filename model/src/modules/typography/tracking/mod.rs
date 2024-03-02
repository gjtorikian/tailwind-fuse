use super::*;

#[derive(Debug, Clone)]
pub struct TailwindTracking {
    kind: StandardValue,
}

impl TailwindInstance for TailwindTracking {
    fn collision_id(&self) -> &'static str {
        "text-tracking"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindTracking {
    /// <https://tailwindcss.com/docs/letter-spacing>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("tracking", &|_| true)(input, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
