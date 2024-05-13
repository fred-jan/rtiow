use std::ops::{Add, Div, Mul, Neg, Sub};

// todo: determine how to implement this without copy/clone (troubleshoot 'cannot move' error)
#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * 1.0 / rhs
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Vec3 {
    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.x * rhs.z - self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit_vector(self) -> Self {
        self.div(self.length())
    }
}

type Point = Vec3;
type Color = Vec3;

fn write_color(color: Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let rbyte = (255.999 * r) as usize;
    let gbyte = (255.999 * g) as usize;
    let bbyte = (255.999 * b) as usize;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    // Render
    (0..image_height).for_each(|y| {
        eprintln!("Scanlines remaining: {}", image_height - y);

        (0..image_width).for_each(|x| {
            let pixel_color = Color {
                x: x as f32 / (image_width - 1) as f32,
                y: y as f32 / (image_height - 1) as f32,
                z: 0f32,
            };
            write_color(pixel_color);
        })
    });

    eprintln!("Done");
}
