
#[derive(Debug, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, PartialEq)]
pub struct HSV {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

impl HSV {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self { h, s, v }
    }
}

impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        let r = rgb.r as f64 / 255.;
        let g = rgb.g as f64 / 255.;
        let b = rgb.b as f64 / 255.;

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);

        let v = max;
        let delta = max - min;
        if delta < 0.00001 {
            return HSV::new(0., 0., v);
        }

        let s = delta / max;

        let mut h;
        if r >= max {
            h = (g - b) / delta;
        } else if g >= max {
            h = 2.0 + (b - r) / delta;
        } else {
            h = 4.0 + (r - g) / delta;
        }
        h *= 60.;

        if h < 0. {
            h += 360.;
        }

        HSV::new(h, s, v)
    }
}

impl From<HSV> for RGB {
    fn from(hsv: HSV) -> Self {
        let s = (hsv.s * 255.) as u8;
        let v = (hsv.v * 255.) as u8;
        if s == 0 {
            return RGB::new(v, v, v);
        }

        let mut hh = hsv.h;
        if hh > 360.0 { hh = 0. };
        hh /= 60.0;
        let i = hh as u32;
        let ff = hh - i as f64;
        let p = ((hsv.v * (1.0 - hsv.s)) * 255.) as u8;
        let q = ((hsv.v * (1.0 - (hsv.s * ff))) * 255.) as u8;
        let t = ((hsv.v * (1.0 - (hsv.s * (1.0 - ff)))) * 255.) as u8;

        let (r, g, b) = match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        RGB::new(r, g, b)
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

        let red_hsv: HSV = red.into();
        let green_hsv: HSV = green.into();
        let blue_hsv: HSV = blue.into();

        assert_eq!(red_hsv, HSV { h: 0., s: 1., v: 1. });
        assert_eq!(green_hsv, HSV { h: 120., s: 1., v: 1. });
        assert_eq!(blue_hsv, HSV { h: 240., s: 1., v: 1. });

        let yellow = RGB::new(255, 255, 0);
        let cyan = RGB::new(0, 255, 255);
        let magenta = RGB::new(255, 0, 255);

        let yellow_hsv: HSV = yellow.into();
        let cyan_hsv: HSV = cyan.into();
        let magenta_hsv: HSV = magenta.into();

        assert_eq!(yellow_hsv, HSV { h: 60., s: 1., v: 1. });
        assert_eq!(cyan_hsv, HSV { h: 180., s: 1., v: 1. });
        assert_eq!(magenta_hsv, HSV { h: 300., s: 1., v: 1. });

        let black = RGB::new(0, 0, 0);
        let white = RGB::new(255, 255, 255);
        let gray = RGB::new(127, 127, 127);

        let black_hsv: HSV = black.into();
        let white_hsv: HSV = white.into();
        let gray_hsv: HSV = gray.into();

        assert_eq!(black_hsv, HSV { h: 0., s: 0., v: 0. });
        assert_eq!(white_hsv, HSV { h: 0., s: 0., v: 1. });
        assert_eq!(gray_hsv.h, 0.);
        assert_eq!(gray_hsv.s, 0.);
        assert!(gray_hsv.v - 0.5 < 0.1);
    }

    #[test]
    fn validate_to_rgb() {
        let red = HSV::new(0., 1., 1.);
        let green = HSV::new(120., 1., 1.);
        let blue = HSV::new(240., 1., 1.);

        let red_rgb: RGB = red.into();
        let green_rgb: RGB = green.into();
        let blue_rgb: RGB = blue.into();

        assert_eq!(red_rgb, RGB::new(255, 0, 0));
        assert_eq!(green_rgb, RGB::new(0, 255, 0));
        assert_eq!(blue_rgb, RGB::new(0, 0, 255));

        let yellow = HSV::new(60., 1., 1.);
        let cyan = HSV::new(180., 1., 1.);
        let magenta = HSV::new(300., 1., 1.);

        let yellow_rgb: RGB = yellow.into();
        let cyan_rgb: RGB = cyan.into();
        let magenta_rgb: RGB = magenta.into();

        assert_eq!(yellow_rgb, RGB::new(255, 255, 0));
        assert_eq!(cyan_rgb, RGB::new(0, 255, 255));
        assert_eq!(magenta_rgb, RGB::new(255, 0, 255));

        let black = HSV::new(0., 0., 0.);
        let white = HSV::new(0., 0., 1.);
        let gray = HSV::new(0., 0., 0.5);

        let black_rgb: RGB = black.into();
        let white_rgb: RGB = white.into();
        let gray_rgb: RGB = gray.into();

        assert_eq!(black_rgb, RGB::new(0, 0, 0));
        assert_eq!(white_rgb, RGB::new(255, 255, 255));
        assert_eq!(gray_rgb, RGB::new(127, 127, 127));
    }
}
