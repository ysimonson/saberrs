use crate::error::{Error, Result};
use std::ops::{Add, Div, Mul, Sub};

pub const RANGE_MAX: i32 = 2047;
pub const RANGE_MIN: i32 = -2047;

macro_rules! match_channel_to {
    ($channel:expr, $ch1:expr, $ch2:expr) => {
        match $channel {
            1 => $ch1,
            2 => $ch2,
            _ => {
                let msg = format!("channel should be 1 or 2 (was {})", $channel);
                return Err(crate::error::Error::InvalidInput(msg));
            }
        }
    };
}

#[cfg(debug_assertions)]
macro_rules! dbg_frame {
    ($head:ident, $frame:expr) => {
        debug!("{} = {:?}", stringify!($head), $frame);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_frame {
    ($head:ident, $frame:expr) => {};
}

pub fn ratio_to_value(ratio: f32) -> Result<i32> {
    if !(-1.0..=1.0).contains(&ratio) {
        return Err(Error::InvalidInput(format!(
            "value ({}) out of range -1.0~1.0",
            ratio
        )));
    }

    let value = (ratio * RANGE_MAX as f32) as i32;

    if value > RANGE_MAX {
        Ok(RANGE_MAX)
    } else if value < RANGE_MIN {
        Ok(RANGE_MIN)
    } else {
        Ok(value)
    }
}

pub fn value_to_ratio(value: i32) -> f32 {
    value as f32 / RANGE_MAX as f32
}

pub fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn checksum(data: &[u8]) -> u8 {
    let s: u32 = data.iter().map(|&b| u32::from(b)).sum();
    (s & 0x7f) as u8
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn test_map_range() {
        assert_delta!(
            map_range((1200.0, 1500.0), (0.0, 1.0), 1200.0f32),
            0.0f32,
            0.001
        );
        assert_delta!(
            map_range((1200.0, 1500.0), (0.0, 1.0), 1350.0f32),
            0.5f32,
            0.001
        );
        assert_delta!(
            map_range((1200.0, 1500.0), (0.0, 1.0), 1500.0f32),
            1.0f32,
            0.001
        );
        assert_delta!(
            map_range((-1.0, 1.0), (-120.0f32, 120.0f32), -1.0),
            -120.0f32,
            0.001
        );
        assert_delta!(
            map_range((-1.0, 1.0), (-120.0f32, 120.0f32), 0.0),
            0.0f32,
            0.001
        );
        assert_delta!(
            map_range((-1.0, 1.0), (-120.0f32, 120.0f32), 1.0),
            120.0f32,
            0.001
        );

        assert_eq!(
            map_range(
                (i8::min_value() as i32, i8::max_value() as i32),
                (204i32, 409i32),
                -128i32,
            ),
            204
        );
        assert_eq!(
            map_range(
                (i8::min_value() as i32, i8::max_value() as i32),
                (204i32, 409i32),
                0i32,
            ),
            306
        );
        assert_eq!(
            map_range(
                (i8::min_value() as i32, i8::max_value() as i32),
                (204i32, 409i32),
                127i32,
            ),
            409
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(0x15, checksum(b"\x80\x81\x04\x07\x09"));
    }
}
