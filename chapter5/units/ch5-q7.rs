#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);


impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-127)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}
impl From<Q7> for f64 {
    fn from(q: Q7) -> f64 {
        (q.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<Q7> for f32 {
    fn from(q: Q7) -> f32 {
        f64::from(q) as f32
    }
}
fn main(){
    let a: f64 = 0.725;
    let b: Q7 = Q7::from(a);
    let c: f64 = f64::from(b);

    println!("a={}, b={:?}, c={}", a, b, c);
}