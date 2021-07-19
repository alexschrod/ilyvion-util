//! Deals with conversions between color spaces

/// Represents an RGB color
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct RGB {
    /// The red component of the color in the range 0-255
    pub red: u8,
    /// The green component of the color in the range 0-255
    pub green: u8,
    /// The blue component of the color in the range 0-255
    pub blue: u8,
}

impl RGB {
    /// Creates a new `RGB` with the given color values
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

/// Represents a HSV color
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HSV {
    /// The hue component of the color in degrees [0-360)
    pub hue: f64,
    /// The saturation component of the color in the range 0.0-1.0
    pub saturation: f64,
    /// The value component of the color in the range 0.0-1.0
    pub value: f64,
}

impl HSV {
    /// Creates a new `HSV` with the given color values
    #[must_use]
    pub fn new(hue: f64, saturation: f64, value: f64) -> Self {
        Self {
            hue,
            saturation,
            value,
        }
    }
}

impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        let red = f64::from(rgb.red) / 255.;
        let green = f64::from(rgb.green) / 255.;
        let blue = f64::from(rgb.blue) / 255.;

        let min = red.min(green).min(blue);
        let max = red.max(green).max(blue);

        let value = max;
        let delta = max - min;
        if delta < 0.00001 {
            return Self::new(0., 0., value);
        }

        let saturation = delta / max;

        let mut hue;
        if red >= max {
            hue = (green - blue) / delta;
        } else if green >= max {
            hue = 2.0 + (blue - red) / delta;
        } else {
            hue = 4.0 + (red - green) / delta;
        }
        hue *= 60.;

        if hue < 0. {
            hue += 360.;
        }

        Self::new(hue, saturation, value)
    }
}

impl From<HSV> for RGB {
    fn from(hsv: HSV) -> Self {
        let saturation = (hsv.saturation * 255.) as u8;
        let value = (hsv.value * 255.) as u8;
        if saturation == 0 {
            return Self::new(value, value, value);
        }

        let mut hue = hsv.hue;
        if hue > 360.0 {
            hue = 0.
        };
        hue /= 60.0;
        let i = hue as u32;
        let ff = hue - f64::from(i);
        let p = ((hsv.value * (1.0 - hsv.saturation)) * 255.) as u8;
        let q = ((hsv.value * (1.0 - (hsv.saturation * ff))) * 255.) as u8;
        let t = ((hsv.value * (1.0 - (hsv.saturation * (1.0 - ff)))) * 255.) as u8;

        let (red, green, blue) = match i {
            0 => (value, t, p),
            1 => (q, value, p),
            2 => (p, value, t),
            3 => (p, q, value),
            4 => (t, p, value),
            _ => (value, p, q),
        };

        Self::new(red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use super::{HSV, RGB};

    #[test]
    fn validate_to_hsv() {
        let red = RGB::new(255, 0, 0);
        let green = RGB::new(0, 255, 0);
        let blue = RGB::new(0, 0, 255);

        let red_hsv = HSV::from(red);
        let green_hsv = HSV::from(green);
        let blue_hsv = HSV::from(blue);

        assert_eq!(
            red_hsv,
            HSV {
                hue: 0.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            green_hsv,
            HSV {
                hue: 120.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            blue_hsv,
            HSV {
                hue: 240.,
                saturation: 1.,
                value: 1.
            }
        );

        let yellow = RGB::new(255, 255, 0);
        let cyan = RGB::new(0, 255, 255);
        let magenta = RGB::new(255, 0, 255);

        let yellow_hsv = HSV::from(yellow);
        let cyan_hsv = HSV::from(cyan);
        let magenta_hsv = HSV::from(magenta);

        assert_eq!(
            yellow_hsv,
            HSV {
                hue: 60.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            cyan_hsv,
            HSV {
                hue: 180.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            magenta_hsv,
            HSV {
                hue: 300.,
                saturation: 1.,
                value: 1.
            }
        );

        let black = RGB::new(0, 0, 0);
        let white = RGB::new(255, 255, 255);
        let gray = RGB::new(127, 127, 127);

        let black_hsv = HSV::from(black);
        let white_hsv = HSV::from(white);
        let gray_hsv = HSV::from(gray);

        assert_eq!(
            black_hsv,
            HSV {
                hue: 0.,
                saturation: 0.,
                value: 0.
            }
        );
        assert_eq!(
            white_hsv,
            HSV {
                hue: 0.,
                saturation: 0.,
                value: 1.
            }
        );
        assert!((gray_hsv.hue - 0.).abs() < f64::EPSILON);
        assert!((gray_hsv.saturation - 0.).abs() < f64::EPSILON);
        assert!(
            (gray_hsv.value - 0.5).abs() < 0.01,
            "{}, {}",
            gray_hsv.value,
            (gray_hsv.value - 0.5).abs()
        );
    }

    #[test]
    fn validate_to_rgb() {
        let red = HSV::new(0., 1., 1.);
        let green = HSV::new(120., 1., 1.);
        let blue = HSV::new(240., 1., 1.);

        let red_rgb = RGB::from(red);
        let green_rgb = RGB::from(green);
        let blue_rgb = RGB::from(blue);

        assert_eq!(red_rgb, RGB::new(255, 0, 0));
        assert_eq!(green_rgb, RGB::new(0, 255, 0));
        assert_eq!(blue_rgb, RGB::new(0, 0, 255));

        let yellow = HSV::new(60., 1., 1.);
        let cyan = HSV::new(180., 1., 1.);
        let magenta = HSV::new(300., 1., 1.);

        let yellow_rgb = RGB::from(yellow);
        let cyan_rgb = RGB::from(cyan);
        let magenta_rgb = RGB::from(magenta);

        assert_eq!(yellow_rgb, RGB::new(255, 255, 0));
        assert_eq!(cyan_rgb, RGB::new(0, 255, 255));
        assert_eq!(magenta_rgb, RGB::new(255, 0, 255));

        let black = HSV::new(0., 0., 0.);
        let white = HSV::new(0., 0., 1.);
        let gray = HSV::new(0., 0., 0.5);

        let black_rgb = RGB::from(black);
        let white_rgb = RGB::from(white);
        let gray_rgb = RGB::from(gray);

        assert_eq!(black_rgb, RGB::new(0, 0, 0));
        assert_eq!(white_rgb, RGB::new(255, 255, 255));
        assert_eq!(gray_rgb, RGB::new(127, 127, 127));
    }
}
