// TODO: CONFIRM USAGES CAN HAVE THESE COLLISION IDS.
macro_rules! keyword_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<String>,
        {
            fn from(input: T) -> Self {
                Self {
                    kind: StandardValue::from(input.into()),
                }
            }
        }
        impl TailwindInstance for $t {
            fn collision_id(&self) -> &'static str {
                $a
            }
            fn get_collisions(&self) -> Vec<&'static str> {
                vec![]
            }
        }
    };
}

macro_rules! color_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<TailwindColor>,
        {
            fn from(color: T) -> Self {
                Self {
                    color: color.into(),
                }
            }
        }
        impl $t {
            pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self {
                    color: TailwindColor::parse(input, arbitrary)?,
                })
            }
            pub fn parse_arbitrary(arbitrary: TailwindArbitrary) -> Result<Self> {
                Ok(Self {
                    color: TailwindColor::Arbitrary(arbitrary),
                })
            }
        }
        impl TailwindInstance for $t {
            fn collision_id(&self) -> &'static str {
                $a
            }
            fn get_collisions(&self) -> Vec<&'static str> {
                vec![]
            }
        }
    };
}

#[macro_export]
macro_rules! syntax_error {
    ($msg:literal $(,)?) => {
        Err(crate::error::TailwindError::syntax_error($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(crate::error::TailwindError::syntax_error(format!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! axisxy_collision {
    ($t:ty => $a:literal) => {
        impl TailwindInstance for $t {
            fn collision_id(&self) -> &'static str {
                match self.axis {
                    AxisXY::N => $a,
                    AxisXY::X => concat!($a, "-x"),
                    AxisXY::Y => concat!($a, "-y"),
                }
            }

            fn get_collisions(&self) -> Vec<&'static str> {
                match self.axis {
                    AxisXY::N => vec![concat!($a, "-x"), concat!($a, "-y")],
                    _ => vec![],
                }
            }
        }
    };
}

#[macro_export]
macro_rules! axis2d_collision {
    ($t:ty => $a:literal) => {
        impl TailwindInstance for $t {
            fn collision_id(&self) -> &'static str {
                match self.axis {
                    Axis2D::X => concat!($a, "-x-dimension"),
                    Axis2D::Y => concat!($a, "-y-dimension"),
                }
            }

            fn get_collisions(&self) -> Vec<&'static str> {
                match self.axis {
                    Axis2D::X => vec![concat!($a, "-dimension")],
                    Axis2D::Y => vec![concat!($a, "-dimension")],
                }
            }
        }
    };
}

#[macro_export]
macro_rules! spacing_collision {
    ($t:ty => $base:literal) => {
        impl $crate::TailwindInstance for $t {
            fn collision_id(&self) -> &'static str {
                match self.axis {
                    SpacingAxis::All => $base,
                    SpacingAxis::X => concat!($base, "-x"),
                    SpacingAxis::Y => concat!($base, "-y"),
                    SpacingAxis::Top => concat!($base, "-top"),
                    SpacingAxis::Right => concat!($base, "-right"),
                    SpacingAxis::Bottom => concat!($base, "-bottom"),
                    SpacingAxis::Left => concat!($base, "-left"),
                }
            }

            fn get_collisions(&self) -> Vec<&'static str> {
                match self.axis {
                    SpacingAxis::All => vec![
                        concat!($base, "-x"),
                        concat!($base, "-y"),
                        concat!($base, "-top"),
                        concat!($base, "-right"),
                        concat!($base, "-bottom"),
                        concat!($base, "-left"),
                    ],
                    SpacingAxis::X => vec![concat!($base, "-left"), concat!($base, "right")],
                    SpacingAxis::Y => vec![concat!($base, "-top"), concat!($base, "-bottom")],
                    _ => vec![],
                }
            }
        }
    };
}

pub(crate) use color_instance;
pub(crate) use keyword_instance;

pub(crate) use axis2d_collision;
pub(crate) use axisxy_collision;
pub(crate) use spacing_collision;
