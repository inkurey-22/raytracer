use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        if length == 0.0 {
            Vec3::default()
        } else {
            Vec3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_vec3_neg() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = -v1;
        assert_eq!(
            v2,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn test_vec3_length() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let l = v1.length();
        assert_eq!(l, (14.0f64).sqrt());
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let d = v1.dot(&v2);
        assert_eq!(d, 32.0);
    }

    #[test]
    fn test_vec3_add() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let v3 = v1 + v2;
        assert_eq!(
            v3,
            Vec3 {
                x: 5.0,
                y: 7.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let v3 = v1 - v2;
        assert_eq!(
            v3,
            Vec3 {
                x: -3.0,
                y: -3.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let v3 = v1 * v2;
        assert_eq!(
            v3,
            Vec3 {
                x: 4.0,
                y: 10.0,
                z: 18.0,
            }
        );
    }

    #[test]
    fn test_vec3_div() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let v3 = v1 / v2;
        assert_eq!(
            v3,
            Vec3 {
                x: 0.25,
                y: 0.4,
                z: 0.5,
            }
        );
    }

    /*#[test]
    fn test_vec3_add_f64() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = v1 + 1.0;
        assert_eq!(
            v2,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }*/

    /*#[test]
    fn test_vec3_sub_f64() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = v1 - 1.0;
        assert_eq!(
            v2,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            }
        );
    }*/

    #[test]
    fn test_vec3_mul_f64() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = v1 * 2.0;
        assert_eq!(
            v2,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn test_vec3_div_f64() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = v1 / 2.0;
        assert_eq!(
            v2,
            Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5,
            }
        );
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        v1 += v2;

        assert_eq!(
            v1,
            Vec3 {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            }
        );
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        v1 -= v2;

        assert_eq!(
            v1,
            Vec3 {
                x: -3.0,
                y: -3.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        v1 *= v2;

        assert_eq!(
            v1,
            Vec3 {
                x: 4.0,
                y: 10.0,
                z: 18.0,
            }
        );
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        v1 /= v2;

        assert_eq!(
            v1,
            Vec3 {
                x: 0.25,
                y: 0.4,
                z: 0.5,
            }
        );
    }

    /*#[test]
    fn test_vec3_add_assign_f64() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        v1 += 1.0;

        assert_eq!(
            v1,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }*/

    /*#[test]
    fn test_vec3_sub_assign_f64() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        v1 -= 1.0;

        assert_eq!(
            v1,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            }
        );
    }*/

    #[test]
    fn test_vec3_mul_assign_f64() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        v1 *= 2.0;

        assert_eq!(
            v1,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn test_vec3_div_assign_f64() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        v1 /= 2.0;

        assert_eq!(
            v1,
            Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5,
            }
        );
    }
}
