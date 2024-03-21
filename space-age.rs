// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
   duration : u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        Duration{
            duration : s
        }
    }
}

pub trait Planet {
    const PERIOD :f64 = 1.0; // those who implement this trait has to provide the value , default is 1 (for earth)
    fn years_during(d: &Duration) -> f64{
        d.duration as f64 /(31557600 as f64 * Self::PERIOD as f64) as f64
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
    const PERIOD :f64 = 0.2408467;
}
impl Planet for Venus {
    const PERIOD :f64 = 0.61519726;
}
impl Planet for Earth{}
impl Planet for Mars {
    const PERIOD :f64 = 1.8808158;
}
impl Planet for Jupiter {
    const PERIOD :f64 = 11.862615;
}
impl Planet for Saturn {
    const PERIOD :f64 = 29.447498;
}
impl Planet for Uranus {
    const PERIOD :f64 = 84.016846;
}
impl Planet for Neptune {
    const PERIOD :f64 = 164.79132;
}
// We'll come back here after the macro chapter from Rust Book. See if we can do better
