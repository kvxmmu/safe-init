use {
    std::{
        marker::PhantomData,
        ops::{ Deref
             , DerefMut },
        mem,

        fmt::{ Debug, Display },
    },

    crate::{ deref::*
           , deref_mut::* }
};

pub struct SafeUninit<T, R> {
    inner: Option<T>,
    phantom: PhantomData<R>
}

impl<T: Display, R> Display for SafeUninit<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.take_ref(), f)
    }
}

impl<T: Debug, R> Debug for SafeUninit<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SafeUninit")
         .field("inner", &self.inner)
         .finish()
    }
}

pub fn test() {
    let x: safe_uninit![_, deref_mut] = SafeUninit::new(-10i32);
    println!("-10 + x = {}", x.take_ref() + 10);
}

impl<T, R> SafeUninit<T, R> {
    /// Check whether the value is initialized or not
    pub fn initialized(&self) -> bool {
        self.inner
            .is_some()
    }

    /// Get inner value of the container
    /// Panics if the value is uninitialized
    pub fn take(self) -> T {
        self.inner
            .unwrap()
    }

    /// Get inner value of the container
    /// Unlike the `SafeUninit::take` method, this method
    /// will not panic if the value is uninitialized
    pub fn take_option(self) -> Option<T> {
        self.inner
    }

    /// Get inner value of the container
    /// Unlike the `SafeUninit::take` method, this method
    /// will return reference to the value
    /// instead of moving out the container
    /// panics if the value is uninitialized
    pub fn take_ref(&self) -> &T {
        self.inner
            .as_ref()
            .unwrap()
    }

    /// Get inner value of the container
    /// Unlike the `SafeUninit::take` method, this method
    /// will return mutable reference to the value
    /// instead of moving out the container
    /// panics if the value is uninitialized
    pub fn take_mut(&mut self) -> &mut T {
        self.inner
            .as_mut()
            .unwrap()
    }

    /// Get inner value of the container
    /// Unlike the `SafeUninit::take` method,
    /// this method do not include initialization checks,
    /// so, accessing unitialized value is an undefined behavior
    pub unsafe fn take_unchecked(self) -> T {
        self.inner
            .unwrap_unchecked()
    }

    /// Get inner value of the container
    /// Same as `SafeUninit::take_mut`, but unlike the `SafeUninit::take_mut` method,
    /// this method do not include initialization checks,
    /// so, accessing unitialized value is an undefined behavior
    pub unsafe fn take_mut_unchecked(&mut self) -> &mut T {
        self.inner
            .as_mut()
            .unwrap_unchecked()
    }
    /// Get inner value of the container
    /// Same as `SafeUninit::take_ref`, but unlike the `SafeUninit::take_ref` method,
    /// this method do not include initialization checks,
    /// so, accessing unitialized value is an undefined behavior
    pub unsafe fn take_ref_unchecked(&self) -> &T {
        self.inner
            .as_ref()
            .unwrap_unchecked()
    }

    /// Exactly the same as `SafeUninit::replace`, but returns a unit type instead
    #[inline(always)]
    pub fn initialize(&mut self, with: T) {
        self.replace(with);
    }

    /// Replace inner value of an container
    /// and return old value
    #[inline(always)]
    pub fn replace(&mut self, with: T) -> Option<T> {
        mem::replace(&mut self.inner, Some(with))
    }

    /// Create uninitialized container
    pub fn uninit() -> Self {
        Self { inner: None
             , phantom: Default::default() }
    }

    /// Create initialized container
    pub fn new(inner: T) -> Self {
        Self { inner: Some(inner)
             , phantom: Default::default() }
    }
}

impl<T, R> Into<Option<T>> for SafeUninit<T, R> {
    fn into(self) -> Option<T> {
        self.take_option()
    }
}

impl<T, R> Default for SafeUninit<T, R>
where T: Default {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, R> DerefMut for SafeUninit<T, R>
where R: EnableDeref + EnableDerefMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
            .as_mut()
            .unwrap()
    }
}

impl<T, R> Deref for SafeUninit<T, R>
where R: EnableDeref {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
            .as_ref()
            .unwrap()
    }
}

pub mod deref;
pub mod deref_mut;

mod macros;

pub use macros::*;
