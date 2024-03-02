use super::*;

#[derive(Clone, Debug)]
pub struct TailwindObjectPosition {
    kind: AnchorPoint,
}

impl TailwindInstance for TailwindObjectPosition {
    fn collision_id(&self) -> &'static str {
        "object-position"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindObjectPosition {
    /// <https://tailwindcss.com/docs/object-fit>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, true)?,
        })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax>
    pub fn check_valid(mode: &str) -> bool {
        AnchorPoint::check_valid(mode)
    }
}
