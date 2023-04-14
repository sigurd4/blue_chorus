#[derive(Clone)]
pub struct TappedDelayLine
{
    pub w: Vec<f32>,
    pub stages: usize,
    pub tap: f32
}

impl TappedDelayLine
{
    pub fn new() -> Self
    {
        Self {
            w: vec![],
            stages: 0,
            tap: 0.0
        }
    }

    pub fn siso(&mut self, x: f32) -> f32
    {
        let i0 = self.tap.floor() as usize;
        let i1 = self.tap.ceil() as usize;
        let p1 = self.tap - self.tap.floor();
        let p0 = 1.0 - p1;

        self.w = (0..self.stages).map(|i|
            if i == 0
            {
                x
            }
            else
            {
                self.w.get(i - 1).map(|xi| *xi).unwrap_or(0.0)
            }
        ).collect();

        return self.w[i0]*p0 + self.w[i1]*p1
    }
}