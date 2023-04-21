#[derive(Debug, Clone)]
pub struct Vec3 {
    a: f32,
    b: f32,
    c: f32,
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vec3 { a, b, c }
    }

    pub fn zeroes() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn scale(&mut self, a: f32) {
        self.map(|b| b * a);
    }

    pub fn add(&mut self, other: &Self) {
        self.combine(other, |a, b| a + b);
    }

    pub fn add_scaled(&mut self, other: &Self, s: f32) {
        self.a += other.a * s;
        self.b += other.b * s;
        self.c += other.c * s;
    }

    pub fn sub(&mut self, other: &Self) {
        self.combine(other, |a, b| a - b);
    }

    pub fn sum(&self) -> f32 {
        self.a + self.b + self.c
    }

    pub fn elwise_mul(&mut self, other: &Self) {
        self.combine(other, |a, b| a * b);
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.a * other.a + self.b * other.b + self.c * other.c
    }

    pub fn map<F: Fn(f32) -> f32>(&mut self, f: F) {
        self.a = f(self.a);
        self.b = f(self.b);
        self.c = f(self.c);
    }

    pub fn combine<F: Fn(f32, f32) -> f32>(&mut self, other: &Self, f: F) {
        self.a = f(self.a, other.a);
        self.b = f(self.b, other.b);
        self.c = f(self.c, other.c)
    }
}
