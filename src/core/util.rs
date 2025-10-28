use std::{fmt::Display, time::{Duration, Instant}};

use crate::{core::Float, PaperClips};

pub fn floor_to(number: Float, power_of_ten: i32) -> Float {
    let factor = 10f64.powi(power_of_ten) as Float;
    (number / factor).floor() * factor
}
pub fn round_to(number: Float, power_of_ten: i32) -> Float {
    let factor = 10f64.powi(power_of_ten) as Float;
    (number / factor).round() * factor
}

pub fn time_cruncher(t: Duration) -> String {
    let secs = t.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;

    let mut string = String::with_capacity(30);

    if h > 0 {
        string.push_str(&h.to_string());
        string.push_str(" hour");
        if h > 1 { string.push('s'); }
        string.push(' ');
    }
    if m > 0 {
        string.push_str(&m.to_string());
        string.push_str(" minute");
        if m > 1 { string.push('s'); }
        string.push(' ');
    }
    if s > 0 {
        string.push_str(&s.to_string());
        string.push_str(" second");
        if s > 1 { string.push('s'); }
        string.push(' ');
    }

    string
}
pub const fn ticks_to_duration(ticks: u128) -> Duration {
    let seconds = (ticks / 100) as u64;
    let nanos = ((ticks % 100) * 10_000_000) as u32;
    Duration::new(seconds, nanos)
}
impl PaperClips {
    pub fn milestone_string(&mut self, milestone: impl Display) -> String {
        format!(
            "{} in {} (REALTIME: {})",
            milestone,
            time_cruncher(ticks_to_duration(self.ticks)),
            time_cruncher(self.session_start.elapsed()),
        )
    }
}

pub const fn powf(mut base: Float, mut exp: u32) -> Float {
    let mut result = 1.0;
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    result
}

pub fn number_cruncher(number: Float, decimals: Option<u8>) -> String {
    let mut number = number; 
    let mut precision = decimals.unwrap_or(2);

    macro_rules! cruncher {
        ($($exp:literal => $name:literal)*) => {
            'a: {
                $(
                    let threshold = powf(10.0, $exp);
                    if number > threshold {
                        number /= threshold;
                        break 'a $name; 
                    }
                )*
                if number < 1000.0 {
                    precision = 0;
                }
                ""
            }
        };
    }

    let suffix = cruncher! {
        51 => "sexdecillion"
        48 => "quindecillion"
        45 => "quattuordecillion"
        42 => "tredecillion"
        39 => "duodecillion"
        36 => "undecillion"
        33 => "decillion"
        30 => "nonillion"
        27 => "octillion"
        24 => "septillion"
        21 => "sextillion"
        18 => "quintillion"
        15 => "quadrillion"
        12 => "trillion"
        9 => "billion"
        6 => "million"
        3 => "thousand"
    };

    if suffix.is_empty() {
        format!("{:.prec$}", number, prec = precision as usize)
    } else {
        format!("{:.prec$} {}", number, suffix, prec = precision as usize)
    }
}

#[inline]
pub const fn ticks(duration: Duration, hertz: Duration) -> u128 {
    duration.as_nanos().div_ceil(hertz.as_nanos())
}

#[inline]
pub const fn ticks_10ms(duration: Duration) -> u128 {
    ticks(duration, Duration::from_millis(10))
}

const BLINK_INTERVAL: u128 = 30;
const MAX_BLINK_DURATION: u128 = BLINK_INTERVAL * 12;

/// Returns if the element should be enabled/normal
pub fn blink(instant: Instant) -> bool {
    let millis = instant.elapsed().as_millis();
    if millis > MAX_BLINK_DURATION {
        return true
    }
    millis / BLINK_INTERVAL % 2 == 1
}
