use std::fmt;
use std::fs;

use color::Color;
use triangle::Triangle;
use vec3::Vec3;

#[derive(Debug, Clone)]
pub struct ObjFile {
    pub center: Vec3,
    pub orientation: Vec3,
    pub path: String,
    pub reflectiveness: f64,
}

impl ObjFile {
    pub fn split_into_triangles(&self) -> Option<Vec<Triangle>> {
        let content = fs::read_to_string(&self.path).ok()?;
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut vertice_color: Vec<Color> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "v" => {
                    if parts.len() >= 7 {
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                            parts[3].parse::<f64>(),
                        ) {
                            vertices.push(Vec3::new(x, y, z));
                        }
                        if let (Ok(r), Ok(g), Ok(b)) = (
                            parts[4].parse::<f64>(),
                            parts[5].parse::<f64>(),
                            parts[6].parse::<f64>(),
                        ) {
                            vertice_color.push(Color::new(r, g, b));
                        }
                    } else if parts.len() >= 4 {
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                            parts[3].parse::<f64>(),
                        ) {
                            vertice_color.push(Color::new(1.0, 1.0, 1.0));
                            vertices.push(Vec3::new(x, y, z));
                        }
                    }
                }
                "f" => {
                    if parts.len() >= 4 {
                        if let (Ok(i1), Ok(i2), Ok(i3)) = (
                            parts[1].parse::<usize>(),
                            parts[2].parse::<usize>(),
                            parts[3].parse::<usize>(),
                        ) {
                            let v0 = vertices.get(i1 - 1)?.clone();
                            let v1 = vertices.get(i2 - 1)?.clone();
                            let v2 = vertices.get(i3 - 1)?.clone();
                            let color = if let (Some(c0), Some(c1), Some(c2)) = (
                                vertice_color.get(i1 - 1),
                                vertice_color.get(i2 - 1),
                                vertice_color.get(i3 - 1),
                            ) {
                                Color::new(
                                    (c0.r + c1.r + c2.r) / 3.0,
                                    (c0.g + c1.g + c2.g) / 3.0,
                                    (c0.b + c1.b + c2.b) / 3.0,
                                )
                            } else {
                                Color::new(1.0, 1.0, 1.0)
                            };
                            fn rotate(v: &Vec3, ori: &Vec3) -> Vec3 {
                                let len = (ori.x * ori.x + ori.y * ori.y + ori.z * ori.z).sqrt();
                                if len == 0.0 {
                                    return *v;
                                }

                                let n = Vec3::new(ori.x / len, ori.y / len, ori.z / len);
                                let up = if n.z.abs() < 0.999 {
                                    Vec3::new(0.0, 0.0, 1.0)
                                } else {
                                    Vec3::new(0.0, 1.0, 0.0)
                                };

                                let t = Vec3::new(
                                    up.y * n.z - up.z * n.y,
                                    up.z * n.x - up.x * n.z,
                                    up.x * n.y - up.y * n.x,
                                );
                                let t_len = (t.x * t.x + t.y * t.y + t.z * t.z).sqrt();
                                let t = Vec3::new(t.x / t_len, t.y / t_len, t.z / t_len);

                                let b = Vec3::new(
                                    n.y * t.z - n.z * t.y,
                                    n.z * t.x - n.x * t.z,
                                    n.x * t.y - n.y * t.x,
                                );

                                Vec3::new(
                                    v.x * t.x + v.y * b.x + v.z * n.x,
                                    v.x * t.y + v.y * b.y + v.z * n.y,
                                    v.x * t.z + v.y * b.z + v.z * n.z,
                                )
                            }

                            let rv0 = rotate(&v0, &self.orientation) + self.center;
                            let rv1 = rotate(&v1, &self.orientation) + self.center;
                            let rv2 = rotate(&v2, &self.orientation) + self.center;

                            triangles.push(Triangle {
                                v0: rv1,
                                v1: rv2,
                                v2: rv0,
                                color,
                                reflectiveness: self.reflectiveness,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        Some(triangles)
    }
}

impl fmt::Display for ObjFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ObjFile")?;
        writeln!(f, "      center: {}", self.center)?;
        writeln!(f, "      orientation: {}", self.orientation)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      path: {}", self.path)
    }
}
