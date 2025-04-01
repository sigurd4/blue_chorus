use crate::parameters::BlueChorusParameters;

#[derive(Clone, Copy, Debug)]
pub struct BlueChorusCache
{
    pub depth: f64,
    pub length: f64,
    pub tap: f64,
    pub rate: f64
}
impl From<&BlueChorusParameters> for BlueChorusCache
{
    fn from(param: &BlueChorusParameters) -> Self
    {
        Self {
            depth: param.depth.get() as f64,
            length: param.length.get() as f64,
            tap: 0.0,
            rate: 44100.0
        }
    }
}