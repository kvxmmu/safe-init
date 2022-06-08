pub(crate) trait EnableDeref {
    
}

// TODO: add usage for this
pub(crate) trait EnableUnsafeDeref {

}

#[derive(Debug)]
pub struct PhantomDeref;
impl EnableDeref for PhantomDeref {}
