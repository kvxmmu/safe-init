pub(crate) trait EnableDerefMut {

}

// TODO: add usage to this
pub(crate) trait EnableUnsafeDerefMut {}

#[derive(Debug)]
pub struct PhantomDerefMut;

impl crate::EnableDeref for PhantomDerefMut {}
impl EnableDerefMut for PhantomDerefMut {}
