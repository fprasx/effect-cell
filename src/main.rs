use effect_cell::EffectCell;

fn main() {
    let mut counter = 0;
    let mut printer = EffectCell::new(0, |_| counter += 1);
    printer <<= 2; // counter increments
    printer <<= 4; // counter increments
    assert_eq!(counter, 2)
}