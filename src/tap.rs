use std::fmt::Debug;

/// Provide some chain call functions such as `tag` in JavaScript.
pub trait Tap<T> {
    /// `tap` functions like it in JavaScript.
    ///
    /// # Examples
    /// ```rust
    /// # use icedragon_utils_rs::prelude::*;
    /// let x = 10usize;
    /// assert_eq!(
    ///     x.tap(|y|println!("{}", y)).abs_diff(30),
    ///     20usize
    /// );
    /// ```
    fn tap<F: FnOnce(&Self)>(self, f: F) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }

    /// Call `dbg!` in chain call.
    ///
    /// # Examples
    /// ```rust
    /// # use icedragon_utils_rs::prelude::*;
    /// let x = 10usize;
    /// assert_eq!(
    ///     x.tap_dbg().abs_diff(30),
    ///     20usize
    /// );
    /// ```
    fn tap_dbg(self) -> Self
    where
        Self: Sized + Debug,
    {
        dbg!(&self);
        self
    }
}

impl<T: Debug> Tap<T> for T {}
