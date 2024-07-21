#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Interval {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Interval {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        Interval {
            min: self.min - delta * 0.5,
            max: self.max + delta * 0.5,
        }
    }

    pub fn new_by_interval(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::universe()
    }
}
