// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    /*
    seconds to divide by can be calculated from the
    mercury_age test case where
    seconds = 2_134_835_688/(280.88 * 0.2408467)
    */
    fn from(s: u64) -> Self {
        return Duration {
            years: s as f64 / 31557525.2,
        };
    }
}

pub trait Planet {
    fn earth_years() -> f64 {
        unimplemented!("return earth years constant",);
    }
    fn years_during(d: &Duration) -> f64 {
        return d.years / Self::earth_years();
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
    fn earth_years() -> f64 {
        return 0.2408467;
    }
}

impl Planet for Venus {
    fn earth_years() -> f64 {
        return 0.61519726;
    }
}

impl Planet for Earth {
    fn earth_years() -> f64 {
        return 1.0;
    }
}

impl Planet for Mars {
    fn earth_years() -> f64 {
        return 1.8808158;
    }
}

impl Planet for Jupiter {
    fn earth_years() -> f64 {
        return 11.862615;
    }
}

impl Planet for Saturn {
    fn earth_years() -> f64 {
        return 29.447498;
    }
}

impl Planet for Uranus {
    fn earth_years() -> f64 {
        return 84.016846;
    }
}

impl Planet for Neptune {
    fn earth_years() -> f64 {
        return 164.79132;
    }
}

//https://stackoverflow.com/a/50223259
// https://exercism.org/tracks/rust/exercises/space-age/solutions/tokenrove

// macro_rules! planet {
//     ($n:ident, $p:expr) => {
//         pub struct $n; impl Planet for $n { fn period() -> f64 { $p } }
//     }
// }
// planet!(Earth, 1.0);
// planet!(Mercury, 0.2408467);
