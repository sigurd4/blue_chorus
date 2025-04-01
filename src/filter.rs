use real_time_fir_iir_filters::{conf::{HighPass, LowPass}, filters::iir::{first::FirstOrderRCFilter, second::SecondOrderSallenKeyFilter}, param::{RC2SallenKey, RC}, rtf::Rtf};

#[derive(Clone, Copy, Debug)]
pub struct FilterChorus
{
    h0: FirstOrderRCFilter<HighPass, f64>,
    h1: FirstOrderRCFilter<LowPass, f64>,
    h2: SecondOrderSallenKeyFilter<LowPass, f64, RC2SallenKey<f64>>,
}

impl FilterChorus
{
    const R9: f64 = 330e3;
    const R10: f64 = 10e3;
    
    const C5: f64 = 33e-9;
    const C6: f64 = 3.3e-9;
    const C7: f64 = 8.2e-9;
    const C8: f64 = 470e-12;

    pub fn filter(&mut self, rate: f64, x: f64) -> f64
    {
        let [x0] = self.h0.filter(rate, x);
        let [x1] = self.h1.filter(rate, x0);
        let [y] = self.h2.filter(rate, x1);
        y
    }
}

impl Default for FilterChorus
{
    fn default() -> Self
    {
        Self {
            h0: FirstOrderRCFilter::new::<HighPass>(RC {r: Self::R9, c: Self::C5}),
            h1: FirstOrderRCFilter::new::<LowPass>(RC {r: Self::R10, c: Self::C6}),
            h2: SecondOrderSallenKeyFilter::new::<LowPass>(RC2SallenKey {r1: Self::R10, c1: Self::C7, r2: Self::R10, c2: Self::C8}),
        }
    }
}