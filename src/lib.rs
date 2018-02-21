//! Bindings to SWI Prolog.
#![warn(missing_docs)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate swipl_sys;

mod atom;
mod functor;

use std::os::raw::c_int;
use std::ptr::null;

pub use atom::Atom;
pub use functor::Functor;

/// An error from SWIPL.
#[derive(Debug, Fail)]
pub enum Error {
    /// An unknown term type was found.
    ///
    /// This is an error in SWI Prolog, swipl-sys, or this crate.
    #[fail(display = "Unknown term type: {}", _0)]
    UnknownTermType(c_int),
}

/// Initializes SWI Prolog.
///
/// This will automatically be called in most cases.
pub fn init() {
    use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

    use swipl_sys::PL_initialise;

    static SWIPL_INITED: AtomicBool = ATOMIC_BOOL_INIT;
    if !SWIPL_INITED.compare_and_swap(false, true, Ordering::SeqCst) {
        let args = [b"swipl\0".as_ptr(), b"-q\0".as_ptr(), null()].as_ptr();
        unsafe { PL_initialise(2, args as *mut *mut i8) };
    }
}
