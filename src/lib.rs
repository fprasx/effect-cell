#![no_std]
use core::ops::{Index, IndexMut, ShlAssign};

/// A container that runs an effect every time its data is mutated.
/// The effect is run with the old data.
///
/// You can mutate an `EffectCell` using the `update` method or the ~fancy~
/// `cell <<= val` syntax.
///
/// `EffectCell` does is also `#![no_std]`!
///
/// ```
/// # use effect_cell::EffectCell;
/// let mut printer = EffectCell::new(1, |x| println!("{}", x));
/// printer <<= 2; // Prints the old data, 1
/// printer <<= 4; // Prints the old data, 2
/// printer.consume(); // Prints 4, consumes the EffectCell
/// ```
///
/// Note: dropping does not execute the effect on the contained value, for this
/// behaviour, use `consume`
pub struct EffectCell<T, E: FnMut(&T)>
where
    E: FnMut(&T),
{
    data: T,
    effect: E,
}

impl<T, E> EffectCell<T, E>
where
    E: FnMut(&T),
{
    /// Create a new `EffectCell` with the provided data and effect.
    pub fn new(t: T, effect: E) -> Self {
        Self { data: t, effect }
    }

    /// Run the effect on the current data.
    pub fn call(&mut self) {
        (self.effect)(&self.data)
    }

    /// Consume the `EffectCell` and perform the effect using the current data.
    /// This may be useful since dropping the `EffectCell` does not perform the
    /// effect.
    pub fn consume(mut self) {
        self.call()
    }

    /// Return the stored data.
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Update the inner value and run the effect using the old value
    /// 
    /// You can also update an `EffectCell` using `<<=`.
    /// ```
    /// # use effect_cell::EffectCell;
    /// let mut updates = 0;
    /// let mut fxl = EffectCell::new(0, |_| updates += 1);
    /// fxl <<= 1;
    /// assert_eq!(updates, 1);
    /// ```
    pub fn update(&mut self, new: T) {
        *self <<= new;
    }

    /// Update the inner value without running the effect.
    pub fn set(&mut self, new: T) {
        self.data = new;
    }
}

impl<T, E> Index<()> for EffectCell<T, E>
where
    E: FnMut(&T),
{
    type Output = T;

    fn index(&self, _: ()) -> &Self::Output {
        &self.data
    }
}

impl<T, E> Clone for EffectCell<T, E>
where
    E: FnMut(&T) + Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            effect: self.effect.clone(),
        }
    }
}

impl<T, E> IndexMut<()> for EffectCell<T, E>
where
    E: FnMut(&T),
{
    fn index_mut(&mut self, _: ()) -> &mut Self::Output {
        (self.effect)(&self.data);
        &mut self.data
    }
}

impl<T, E> ShlAssign<T> for EffectCell<T, E>
where
    E: FnMut(&T),
{
    fn shl_assign(&mut self, rhs: T) {
        self.call();
        self.data = rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_not_run_on_drop() {
        let mut counter = 0;
        let fxl = EffectCell::new(0, |_| counter += 1);
        #[allow(clippy::drop_non_drop)]
        drop(fxl);
        assert_eq!(counter, 0)
    }
}
