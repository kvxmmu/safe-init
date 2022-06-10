/// Easy way to access to the SafeUninit type
/// 
/// SafeUninit![T] = SafeUninit![T, deref_mut]
/// 
/// SafeUninit![T, deref] - implement only Deref trait, no DerefMut
/// 
/// SafeUninit![T, deref_mut] - implement Deref + DerefMut traits
/// 
/// This makes SafeUninit struct transparent to its underlying value
#[macro_export]
macro_rules! SafeUninit {
    ($ty:ty) => {
        $crate::safe_uninit![$ty, deref_mut]
    };

    ($ty:ty, no_deref) => {
        $crate::SafeUninit::<$ty, ()>
    };

    ($ty:ty, deref) => {
        $crate::SafeUninit::<$ty, $crate::deref::PhantomDeref>
    };

    ($ty:ty, deref_mut) => {
        $crate::SafeUninit::<$ty, $crate::deref_mut::PhantomDerefMut>
    };
}
