//! Deals with conversions between color spaces

/// Represents an RGB color
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rgb {
    /// The red component of the color in the range 0-255
    pub red: u8,
    /// The green component of the color in the range 0-255
    pub green: u8,
    /// The blue component of the color in the range 0-255
    pub blue: u8,
}

impl Rgb {
    /// Creates a new `RGB` with the given color values
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

/// Represents a HSV color
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Hsv {
    /// The hue component of the color in degrees [0-360)
    pub hue: f64,
    /// The saturation component of the color in the range 0.0-1.0
    pub saturation: f64,
    /// The value component of the color in the range 0.0-1.0
    pub value: f64,
}

impl Hsv {
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

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
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

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        let saturation = (hsv.saturation * 255.) as u8;
        let value = (hsv.value * 255.) as u8;
        if saturation == 0 {
            return Self::new(value, value, value);
        }

        let mut hue = hsv.hue;
        if hue > 360.0 {
            hue = 0.;
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
    use super::{Hsv, Rgb};

    #[test]
    fn validate_to_hsv() {
        let red = Rgb::new(255, 0, 0);
        let green = Rgb::new(0, 255, 0);
        let blue = Rgb::new(0, 0, 255);

        let red_hsv = Hsv::from(red);
        let green_hsv = Hsv::from(green);
        let blue_hsv = Hsv::from(blue);

        assert_eq!(
            red_hsv,
            Hsv {
                hue: 0.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            green_hsv,
            Hsv {
                hue: 120.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            blue_hsv,
            Hsv {
                hue: 240.,
                saturation: 1.,
                value: 1.
            }
        );

        let yellow = Rgb::new(255, 255, 0);
        let cyan = Rgb::new(0, 255, 255);
        let magenta = Rgb::new(255, 0, 255);

        let yellow_hsv = Hsv::from(yellow);
        let cyan_hsv = Hsv::from(cyan);
        let magenta_hsv = Hsv::from(magenta);

        assert_eq!(
            yellow_hsv,
            Hsv {
                hue: 60.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            cyan_hsv,
            Hsv {
                hue: 180.,
                saturation: 1.,
                value: 1.
            }
        );
        assert_eq!(
            magenta_hsv,
            Hsv {
                hue: 300.,
                saturation: 1.,
                value: 1.
            }
        );

        let black = Rgb::new(0, 0, 0);
        let white = Rgb::new(255, 255, 255);
        let gray = Rgb::new(127, 127, 127);

        let black_hsv = Hsv::from(black);
        let white_hsv = Hsv::from(white);
        let gray_hsv = Hsv::from(gray);

        assert_eq!(
            black_hsv,
            Hsv {
                hue: 0.,
                saturation: 0.,
                value: 0.
            }
        );
        assert_eq!(
            white_hsv,
            Hsv {
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
        let red = Hsv::new(0., 1., 1.);
        let green = Hsv::new(120., 1., 1.);
        let blue = Hsv::new(240., 1., 1.);

        let red_rgb = Rgb::from(red);
        let green_rgb = Rgb::from(green);
        let blue_rgb = Rgb::from(blue);

        assert_eq!(red_rgb, Rgb::new(255, 0, 0));
        assert_eq!(green_rgb, Rgb::new(0, 255, 0));
        assert_eq!(blue_rgb, Rgb::new(0, 0, 255));

        let yellow = Hsv::new(60., 1., 1.);
        let cyan = Hsv::new(180., 1., 1.);
        let magenta = Hsv::new(300., 1., 1.);

        let yellow_rgb = Rgb::from(yellow);
        let cyan_rgb = Rgb::from(cyan);
        let magenta_rgb = Rgb::from(magenta);

        assert_eq!(yellow_rgb, Rgb::new(255, 255, 0));
        assert_eq!(cyan_rgb, Rgb::new(0, 255, 255));
        assert_eq!(magenta_rgb, Rgb::new(255, 0, 255));

        let black = Hsv::new(0., 0., 0.);
        let white = Hsv::new(0., 0., 1.);
        let gray = Hsv::new(0., 0., 0.5);

        let black_rgb = Rgb::from(black);
        let white_rgb = Rgb::from(white);
        let gray_rgb = Rgb::from(gray);

        assert_eq!(black_rgb, Rgb::new(0, 0, 0));
        assert_eq!(white_rgb, Rgb::new(255, 255, 255));
        assert_eq!(gray_rgb, Rgb::new(127, 127, 127));
    }
}
