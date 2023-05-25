#![doc = include_str!("../README.md")]
use core::fmt::Debug;
use core::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};

/// A container that runs one or many effects on data mutation.
/// The effect is run after data is updated as per the conventions of the Observer data structure.
///
/// # Examples
///
/// ```
/// use effect_cell::EffectCell;
///
/// let mut effect_cell = EffectCell::new(0);
/// effect_cell.bind(|data| { println!("{data}"); });
/// effect_cell.update(2);
/// ```
pub struct EffectCell<T> {
    data: T,
    effects: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> EffectCell<T> {
    /// Create a new [`EffectCell`] with the provided data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            effects: Vec::new(),
        }
    }

    /// Returns the stored data.
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Binds a new effect callback to the [`EffectCell`]
    pub fn bind<F: FnMut(&T) + 'static>(&mut self, effect: F) {
        self.effects.push(Box::new(effect));
    }

    /// Runs all effects with the current data.
    pub fn call(&mut self) {
        for f in &mut self.effects {
            f(&self.data);
        }
    }

    /// Consume the [`EffectCell`] and performs all effects using the current data.
    /// This may be useful since dropping the [`EffectCell`] does not perform
    /// effects.
    pub fn consume(mut self) {
        self.call();
    }

    /// Updates the inner value and runs the effects with the new value.
    ///
    /// # Examples
    ///
    /// ```
    /// use effect_cell::EffectCell;
    ///
    /// let mut effect_cell = EffectCell::new(0);
    /// effect_cell.bind(|data| { println!("{data}"); });
    /// effect_cell.update(2);
    /// ```
    pub fn update(&mut self, new: T) {
        self.data = new;
        self.call();
    }

    /// Updates the inner value using the provided function and runs the effects with the new value.
    ///
    /// # Examples
    ///
    /// ```
    /// use effect_cell::EffectCell;
    ///
    /// let mut effect_cell = EffectCell::new(0);
    /// effect_cell.bind(|data| { println!("{data}"); });
    /// effect_cell.update_lambda(|data| *data += 1);
    ///
    /// assert_eq!(effect_cell.into_inner(), 1);
    /// ```
    pub fn update_lambda<F: FnMut(&mut T) + 'static>(&mut self, mut lambda: F) {
        lambda(&mut self.data);
        self.call();
    }

    /// Updates the inner value without running any effects
    pub fn set(&mut self, new: T) {
        self.data = new;
    }

    /// Updates the inner value using the provided function without running any effects
    pub fn set_lambda<F: FnMut(&T) + 'static>(&mut self, mut lambda: F) {
        lambda(&mut self.data);
    }
}

impl<T> PartialEq for EffectCell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> PartialEq<T> for EffectCell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.data == other
    }
}

impl<T> PartialOrd for EffectCell<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T> PartialOrd<T> for EffectCell<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl<T: Debug> Debug for EffectCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectCell")
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

/// A container that runs one or many effects on data mutation.
/// Effects are run either before or after data is updated depending on their [`EffectOrder`]
///
/// # Examples
///
/// ```
/// # fn main() {
/// use effect_cell::OrderedEffectCell;
/// use effect_cell::EffectOrder;
///
/// let mut ordered_effect_cell = OrderedEffectCell::new(0);
/// ordered_effect_cell.bind(
///     EffectOrder::Prior,
///     |data| { println!("Value before: {data}"); }
/// );
/// ordered_effect_cell.bind(
///     EffectOrder::Post,
///     |data| { println!("Value after: {data}"); }
/// );
/// ordered_effect_cell.update(2);
///
/// // Prints the following:
/// // Value before: 0
/// // Value after: 2
/// # }
/// ```
pub struct OrderedEffectCell<T> {
    data: T,
    prior_effects: Vec<Box<dyn FnMut(&T)>>,
    post_effects: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> OrderedEffectCell<T> {
    /// Create a new [`OrderedEffectCell`] with the provided data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            prior_effects: Vec::new(),
            post_effects: Vec::new(),
        }
    }

    /// Returns the stored data.
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Binds a new effect callback to [`OrderedEffectCell`] based on the given [`EffectOrder`]
    pub fn bind<F: FnMut(&T) + 'static>(&mut self, ord: EffectOrder, effect: F) {
        match ord {
            EffectOrder::Prior => {
                self.prior_effects.push(Box::new(effect));
            }
            EffectOrder::Post => {
                self.post_effects.push(Box::new(effect));
            }
        }
    }

    /// Runs all effects of the given [`EffectOrder`]
    pub fn call(&mut self, ord: EffectOrder) {
        match ord {
            EffectOrder::Prior => {
                for f in &mut self.prior_effects {
                    f(&self.data);
                }
            }
            EffectOrder::Post => {
                for f in &mut self.post_effects {
                    f(&self.data);
                }
            }
        }
    }

    /// Consume the [`OrderedEffectCell`] and performs all effects using the current data.
    /// This may be useful since dropping the [`OrderedEffectCell`] does not perform
    /// effects.
    ///
    /// Both `Prior` and `Post` effects are called.
    pub fn consume(mut self) {
        self.call(EffectOrder::Prior);
        self.call(EffectOrder::Post);
    }

    /// Updates inner value and runs effects with either the new or old value dependent on their
    /// [`EffectOrder`]
    ///
    /// # Examples
    ///
    /// ```
    /// use effect_cell::OrderedEffectCell;
    /// use effect_cell::EffectOrder;
    ///
    /// let mut ordered_effect_cell = OrderedEffectCell::new(0);
    /// ordered_effect_cell.bind(
    ///     EffectOrder::Prior,
    ///     |data| { println!("Value before: {data}"); }
    /// );
    /// ordered_effect_cell.bind(
    ///     EffectOrder::Post,
    ///     |data| { println!("Value after: {data}"); }
    /// );
    /// ordered_effect_cell.update(2);
    ///
    /// // Prints the following:
    /// // Value before: 0
    /// // Value after: 2
    /// ```
    pub fn update(&mut self, new: T) {
        self.call(EffectOrder::Prior);
        self.data = new;
        self.call(EffectOrder::Post);
    }

    /// Updates inner value using the provided function and runs effects with either the new or
    /// old value dependent on their [`EffectOrder`]
    ///
    /// # Examples
    ///
    /// ```
    /// use effect_cell::OrderedEffectCell;
    /// use effect_cell::EffectOrder;
    ///
    /// let mut ordered_effect_cell = OrderedEffectCell::new(0);
    /// ordered_effect_cell.bind(
    ///     EffectOrder::Prior,
    ///     |data| { println!("Value before: {data}"); }
    /// );
    /// ordered_effect_cell.bind(
    ///     EffectOrder::Post,
    ///     |data| { println!("Value after: {data}"); }
    /// );
    /// ordered_effect_cell.update_lambda(|data| *data += 1);
    ///
    /// // Prints the following:
    /// // Value before: 0
    /// // Value after: 1
    ///
    /// assert_eq!(ordered_effect_cell.into_inner(), 1);
    /// ```
    pub fn update_lambda<F: FnMut(&mut T) + 'static>(&mut self, mut lambda: F) {
        self.call(EffectOrder::Prior);
        lambda(&mut self.data);
        self.call(EffectOrder::Post);
    }

    /// Updates the inner value without running any effects
    pub fn set(&mut self, new: T) {
        self.data = new;
    }

    /// Updates the inner value using the provided function without running any effects
    pub fn set_lambda<F: FnMut(&T) + 'static>(&mut self, mut lambda: F) {
        lambda(&mut self.data);
    }
}

impl<T> PartialEq for OrderedEffectCell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> PartialEq<T> for OrderedEffectCell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.data == other
    }
}

impl<T> PartialOrd for OrderedEffectCell<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T> PartialOrd<T> for OrderedEffectCell<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl<T: Debug> Debug for OrderedEffectCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrderedEffectCell")
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

macro_rules! impl_pass_op {
    ($struct_name:ty, $trait:path, $trait_name:path, $fn_name:ident, $op:tt) => {
        impl<T> $trait for $struct_name
        where
            T: $trait_name,
        {
            fn $fn_name(&mut self, other: T) {
                self.data $op other;
                self.call()
            }
        }
    }
}

macro_rules! impl_pass_op_ord {
    ($struct_name:ty, $trait:path, $trait_name:path, $fn_name:ident, $op:tt) => {
        impl<T> $trait for $struct_name
        where
            T: $trait_name,
        {
            fn $fn_name(&mut self, other: T) {
                self.call(EffectOrder::Prior);
                self.data $op other;
                self.call(EffectOrder::Post);
            }
        }
    }
}

macro_rules! impl_struct_pass {
    ($struct_name:ty) => {
        impl_pass_op!($struct_name, AddAssign<T>, AddAssign, add_assign, +=);
        impl_pass_op!($struct_name, SubAssign<T>, SubAssign, sub_assign, -=);
        impl_pass_op!($struct_name, MulAssign<T>, MulAssign, mul_assign, *=);
        impl_pass_op!($struct_name, DivAssign<T>, DivAssign, div_assign, /=);
        impl_pass_op!($struct_name, RemAssign<T>, RemAssign, rem_assign, %=);
        impl_pass_op!($struct_name, ShlAssign<T>, ShlAssign, shl_assign, <<=);
        impl_pass_op!($struct_name, ShrAssign<T>, ShrAssign, shr_assign, >>=);
        impl_pass_op!($struct_name, BitAndAssign<T>, BitAndAssign, bitand_assign, &=);
        impl_pass_op!($struct_name, BitOrAssign<T>, BitOrAssign, bitor_assign, |=);
        impl_pass_op!($struct_name, BitXorAssign<T>, BitXorAssign, bitxor_assign, ^=);
    };
}

macro_rules! impl_struct_pass_ord {
    ($struct_name:ty) => {
        impl_pass_op_ord!($struct_name, AddAssign<T>, AddAssign, add_assign, +=);
        impl_pass_op_ord!($struct_name, SubAssign<T>, SubAssign, sub_assign, -=);
        impl_pass_op_ord!($struct_name, MulAssign<T>, MulAssign, mul_assign, *=);
        impl_pass_op_ord!($struct_name, DivAssign<T>, DivAssign, div_assign, /=);
        impl_pass_op_ord!($struct_name, RemAssign<T>, RemAssign, rem_assign, %=);
        impl_pass_op_ord!($struct_name, ShlAssign<T>, ShlAssign, shl_assign, <<=);
        impl_pass_op_ord!($struct_name, ShrAssign<T>, ShrAssign, shr_assign, >>=);
        impl_pass_op_ord!($struct_name, BitAndAssign<T>, BitAndAssign, bitand_assign, &=);
        impl_pass_op_ord!($struct_name, BitOrAssign<T>, BitOrAssign, bitor_assign, |=);
        impl_pass_op_ord!($struct_name, BitXorAssign<T>, BitXorAssign, bitxor_assign, ^=);
    };
}

impl_struct_pass!(EffectCell<T>);
impl_struct_pass_ord!(OrderedEffectCell<T>);

/// Represents whether an effect should be called before or after data is updated
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectOrder {
    /// Represents the ordering of an effect that should be called before data is updated
    Prior,
    /// Represents the ordering of an effect the should be called after data is updated
    Post,
}
