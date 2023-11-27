pub mod ascii_num;
use ascii_num::ONE;
use rand::{Rng, thread_rng};

fn main() {
    fn zero_one() -> bool {
        let mut seed = thread_rng();
        return seed.gen::<bool>()
    }
    fn segment01(zron: bool) -> &'static str {
        if zron == false {
            return ascii_num::ZERO;
        } else {
            return ONE;
        }
    } 
    println!(r#"{}"#,segment01(zero_one()));
}