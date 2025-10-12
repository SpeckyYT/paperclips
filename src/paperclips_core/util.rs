use std::time::Duration;

use crate::paperclips_core::Float;

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

const fn powf(base: Float, exp: u32) -> Float {
    let mut result = 1.0;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            result *= b;
        }
        b *= b;
        e /= 2;
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
