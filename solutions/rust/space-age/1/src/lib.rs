const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.240_846_7;
}

impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.615_197_26;
}

impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}

impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.880_815_8;
}

impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862_615;
}

impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447_498;
}

impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016_846;
}

impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.791_32;
}
