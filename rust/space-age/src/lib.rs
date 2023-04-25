// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    y: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            y: s as f64 / 31557600.,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! create_planet {
    ( $($name:ident: $period:literal),+ ) => {
        $(
            pub struct $name;
            impl Planet for $name {
                fn years_during(d: &Duration) -> f64 {
                    d.y / $period
                }
            }
        )*
    };
}

create_planet!(
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1.0,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune: 164.79132
);
