use std::collections::VecDeque;

#[derive(Clone)]
pub struct TappedDelayLine
{
    pub w: VecDeque<f64>,
    pub stages: usize,
    pub tap: f64
}

impl TappedDelayLine
{
    pub fn new() -> Self
    {
        Self {
            w: VecDeque::new(),
            stages: 0,
            tap: 0.0
        }
    }

    pub fn delay(&mut self, x: f64) -> f64
    {
        let i0 = self.tap.floor() as usize;
        let i1 = self.tap.ceil() as usize;
        let p1 = self.tap - self.tap.floor();
        let p0 = 1.0 - p1;

        self.stages = self.stages.max(i1);

        self.w.truncate(self.stages);
        self.w.push_front(x);
        while self.w.len() > self.stages
        {
            self.w.pop_back();
        }

        self.w.get(i0).map(|&w| w).unwrap_or(0.0)*p0 + self.w.get(i1).map(|&w| w).unwrap_or(0.0)*p1
    }
}