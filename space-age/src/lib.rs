// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
/*
- Mercury: orbital period 0.2408467 Earth years
- Venus: orbital period 0.61519726 Earth years
- Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
- Mars: orbital period 1.8808158 Earth years
- Jupiter: orbital period 11.862615 Earth years
- Saturn: orbital period 29.447498 Earth years
- Uranus: orbital period 84.016846 Earth years
- Neptune: orbital period 164.79132 Earth years
 */
const EARTH_YEAR_SECONDS: f64 = 31557600.0;
const MERCURY_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 0.2408467;
const VENUS_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 0.61519726;
const MARS_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 1.8808158;
const JUPITER_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 11.862615;
const SATURN_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 29.447498;
const URANUS_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 84.016846;
const NEPTUNE_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * 164.79132;


#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}


macro_rules! planet_impl {
    ($planet_type:ident, $orbital_period:expr) => {
        pub struct $planet_type;

        impl Planet for $planet_type {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / $orbital_period
            }
        }
    };
}


planet_impl!(Mercury, MERCURY_YEAR_SECONDS);
planet_impl!(Venus, VENUS_YEAR_SECONDS);
planet_impl!(Earth, EARTH_YEAR_SECONDS);
planet_impl!(Mars, MARS_YEAR_SECONDS);
planet_impl!(Jupiter, JUPITER_YEAR_SECONDS);
planet_impl!(Saturn, SATURN_YEAR_SECONDS);
planet_impl!(Uranus, URANUS_YEAR_SECONDS);
planet_impl!(Neptune, NEPTUNE_YEAR_SECONDS);