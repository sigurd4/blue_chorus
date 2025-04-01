use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct TappedDelayLine
{
    pub w: VecDeque<f64>,
    pub tap: f64
}

impl TappedDelayLine
{
    pub fn delay(&mut self, x: f64, length: usize) -> f64
    {
        let i0 = self.tap.floor() as usize;
        let i1 = i0 + 1;
        let p = self.tap.fract();
        let q = 1.0 - p;

        self.w.push_front(x);
        self.w.truncate(length);

        self.w.get(i0).map(|&x| x*q).unwrap_or(0.0)
            + self.w.get(i1).map(|&x| x*p).unwrap_or(0.0)
    }
}

impl Default for TappedDelayLine
{
    fn default() -> Self
    {
        Self {
            w: VecDeque::new(),
            tap: 0.0
        }
    }
}