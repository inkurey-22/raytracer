use std::fmt;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use vec3::Vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Orientation {
    pub p: f64, // pitch
    pub y: f64, // yaw
    pub r: f64, // roll
}

fn limit_angle(angle: f64) -> f64 {
    let mut a = angle % 360.0;
    while a < 0.0 {
        a += 360.0;
    }
    a
}

impl Orientation {
    pub const fn new(p: f64, y: f64, r: f64) -> Self {
        Self { p, y, r }
    }

    pub fn into_vec3(self, length: f64) -> Vec3 {
        let p_rad = self.p.to_radians();
        let y_rad = self.y.to_radians();

        let x = length * p_rad.cos() * y_rad.cos();
        let y = length * p_rad.cos() * y_rad.sin();
        let z = length * p_rad.sin();

        Vec3::new(x, y, z)
    }

    pub fn limit(&mut self) {
        self.p = limit_angle(self.p);
        self.y = limit_angle(self.y);
        self.r = limit_angle(self.r);
    }
}

pub trait Vec3OrientationExt {
    fn into_orientation(self) -> Orientation;
}

impl Vec3OrientationExt for Vec3 {
    fn into_orientation(self) -> Orientation {
        if self.length() == 0.0 {
            return Orientation::new(0.0, 0.0, 0.0);
        }
        let p = self
            .z
            .atan2((self.x * self.x + self.y * self.y).sqrt())
            .to_degrees();
        let y = self.y.atan2(self.x).to_degrees();
        let r = 0.0;

        Orientation::new(p, y, r)
    }
}

impl Add for Orientation {
    type Output = Orientation;

    fn add(self, rhs: Self) -> Self::Output {
        Orientation {
            p: limit_angle(self.p + rhs.p),
            y: limit_angle(self.y + rhs.y),
            r: limit_angle(self.r + rhs.r),
        }
    }
}

impl AddAssign for Orientation {
    fn add_assign(&mut self, rhs: Self) {
        self.p += rhs.p;
        self.y += rhs.y;
        self.r += rhs.r;
        self.limit();
    }
}

impl Sub for Orientation {
    type Output = Orientation;

    fn sub(self, rhs: Self) -> Self::Output {
        Orientation {
            p: limit_angle(self.p - rhs.p),
            y: limit_angle(self.y - rhs.y),
            r: limit_angle(self.r - rhs.r),
        }
    }
}

impl SubAssign for Orientation {
    fn sub_assign(&mut self, rhs: Self) {
        self.p -= rhs.p;
        self.y -= rhs.y;
        self.r -= rhs.r;
        self.limit();
    }
}

impl Neg for Orientation {
    type Output = Orientation;

    fn neg(self) -> Self::Output {
        Orientation {
            p: limit_angle(self.p + 180.0),
            y: limit_angle(self.y + 180.0),
            r: limit_angle(self.r + 180.0),
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.p, self.y, self.r)
    }
}

#[cfg(test)]
mod tests {
    use super::Orientation;
    use super::Vec3OrientationExt;

    #[test]
    fn orientation_neg() {
        let v1 = Orientation {
            p: 90.0,
            y: 0.0,
            r: 180.0,
        };
        let v2 = -v1;
        assert_eq!(
            v2,
            Orientation {
                p: 270.0,
                y: 180.0,
                r: 0.0,
            }
        );
    }

    #[test]
    fn limit_orientation() {
        let mut v1 = Orientation {
            p: 450.0,
            y: -90.0,
            r: 720.0,
        };
        v1.limit();
        assert_eq!(
            v1,
            Orientation {
                p: 90.0,
                y: 270.0,
                r: 0.0,
            }
        );
    }

    #[test]
    fn vec3_into_orientation() {
        let v1 = vec3::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let o1 = v1.into_orientation();
        assert_eq!(
            o1,
            Orientation {
                p: 0.0,
                y: 90.0,
                r: 0.0,
            }
        );
    }

    #[test]
    fn orientation_into_vec3() {
        let o1 = Orientation {
            p: 90.0,
            y: 0.0,
            r: 0.0,
        };
        let v1 = o1.into_vec3(1.0);
        assert_eq!(
            v1,
            vec3::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
    }

    #[test]
    fn orientation_add() {
        let v1 = Orientation {
            p: 180.0,
            y: 0.0,
            r: 0.0,
        };
        let v2 = Orientation {
            p: 180.0,
            y: 90.0,
            r: 361.0,
        };
        let v3 = v1 + v2;
        assert_eq!(
            v3,
            Orientation {
                p: 0.0,
                y: 90.0,
                r: 1.0
            }
        );
    }

    #[test]
    fn orientation_sub() {
        let v1 = Orientation {
            p: 0.0,
            y: 90.0,
            r: 360.0,
        };
        let v2 = Orientation {
            p: 1.0,
            y: 90.0,
            r: 720.0,
        };
        let v3 = v1 - v2;
        assert_eq!(
            v3,
            Orientation {
                p: 359.0,
                y: 0.0,
                r: 0.0,
            }
        );
    }

    #[test]
    fn orientation_add_assign() {
        let mut v1 = Orientation {
            p: 180.0,
            y: 0.0,
            r: 0.0,
        };
        let v2 = Orientation {
            p: 180.0,
            y: 90.0,
            r: 361.0,
        };

        v1 += v2;

        assert_eq!(
            v1,
            Orientation {
                p: 0.0,
                y: 90.0,
                r: 1.0,
            }
        );
    }

    #[test]
    fn orientation_sub_assign() {
        let mut v1 = Orientation {
            p: 0.0,
            y: 90.0,
            r: 360.0,
        };
        let v2 = Orientation {
            p: 1.0,
            y: 90.0,
            r: 720.0,
        };

        v1 -= v2;

        assert_eq!(
            v1,
            Orientation {
                p: 359.0,
                y: 0.0,
                r: 0.0,
            }
        );
    }
}
