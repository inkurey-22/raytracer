use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn normalize_max(&self) -> Color {
        let max_val = self.r.max(self.g).max(self.b);
        if max_val > 1.0 {
            Color {
                r: self.r / max_val,
                g: self.g / max_val,
                b: self.b / max_val,
            }
        } else {
            *self
        }
    }

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn dot(&self, other: &Color) -> f64 {
        self.r * other.r + self.g * other.g + self.b * other.b
    }

    pub fn cross(&self, other: &Color) -> Color {
        Color {
            r: self.g * other.b - self.b * other.g,
            g: self.b * other.r - self.r * other.b,
            b: self.r * other.g - self.g * other.r,
        }
    }

    pub fn saturate(&self) -> Color {
        Color {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl DivAssign for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_color_neg() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = -v1;
        assert_eq!(
            v2,
            Color {
                r: -1.0,
                g: -2.0,
                b: -3.0,
            }
        );
    }

    #[test]
    fn test_color_cross() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let v3 = v1.cross(&v2);
        assert_eq!(
            v3,
            Color {
                r: -3.0,
                g: 6.0,
                b: -3.0,
            }
        );
    }

    #[test]
    fn test_color_dot() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let d = v1.dot(&v2);
        assert_eq!(d, 32.0);
    }

    #[test]
    fn test_color_saturate() {
        let v1 = Color {
            r: -0.5,
            g: 0.5,
            b: 1.5,
        };
        let v2 = v1.saturate();
        assert_eq!(
            v2,
            Color {
                r: 0.0,
                g: 0.5,
                b: 1.0,
            }
        );
    }

    #[test]
    fn test_color_add() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let v3 = v1 + v2;
        assert_eq!(
            v3,
            Color {
                r: 5.0,
                g: 7.0,
                b: 9.0
            }
        );
    }

    #[test]
    fn test_color_sub() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let v3 = v1 - v2;
        assert_eq!(
            v3,
            Color {
                r: -3.0,
                g: -3.0,
                b: -3.0,
            }
        );
    }

    #[test]
    fn test_color_mul() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let v3 = v1 * v2;
        assert_eq!(
            v3,
            Color {
                r: 4.0,
                g: 10.0,
                b: 18.0,
            }
        );
    }

    #[test]
    fn test_color_div() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };
        let v3 = v1 / v2;
        assert_eq!(
            v3,
            Color {
                r: 0.25,
                g: 0.4,
                b: 0.5,
            }
        );
    }

    #[test]
    fn test_color_mul_f64() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = v1 * 2.0;
        assert_eq!(
            v2,
            Color {
                r: 2.0,
                g: 4.0,
                b: 6.0,
            }
        );
    }

    #[test]
    fn test_color_div_f64() {
        let v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = v1 / 2.0;
        assert_eq!(
            v2,
            Color {
                r: 0.5,
                g: 1.0,
                b: 1.5,
            }
        );
    }

    #[test]
    fn test_color_add_assign() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };

        v1 += v2;

        assert_eq!(
            v1,
            Color {
                r: 5.0,
                g: 7.0,
                b: 9.0,
            }
        );
    }

    #[test]
    fn test_color_sub_assign() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };

        v1 -= v2;

        assert_eq!(
            v1,
            Color {
                r: -3.0,
                g: -3.0,
                b: -3.0,
            }
        );
    }

    #[test]
    fn test_color_mul_assign() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };

        v1 *= v2;

        assert_eq!(
            v1,
            Color {
                r: 4.0,
                g: 10.0,
                b: 18.0,
            }
        );
    }

    #[test]
    fn test_color_div_assign() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let v2 = Color {
            r: 4.0,
            g: 5.0,
            b: 6.0,
        };

        v1 /= v2;

        assert_eq!(
            v1,
            Color {
                r: 0.25,
                g: 0.4,
                b: 0.5,
            }
        );
    }

    #[test]
    fn test_color_mul_assign_f64() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };

        v1 *= 2.0;

        assert_eq!(
            v1,
            Color {
                r: 2.0,
                g: 4.0,
                b: 6.0,
            }
        );
    }

    #[test]
    fn test_color_div_assign_f64() {
        let mut v1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };

        v1 /= 2.0;

        assert_eq!(
            v1,
            Color {
                r: 0.5,
                g: 1.0,
                b: 1.5,
            }
        );
    }
}
