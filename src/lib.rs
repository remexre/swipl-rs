//! Bindings to SWI Prolog.
#![warn(missing_docs)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate swipl_sys;

mod atom;
mod functor;

use std::cell::RefCell;
use std::os::raw::c_int;
use std::ptr::{null, null_mut};
use std::sync::Mutex;

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
    use swipl_sys::{PL_initialise, PL_thread_attach_engine};

    lazy_static!{
        static ref SWIPL_INITED_GLOBAL: Mutex<bool> = Mutex::new(false);
    }
    thread_local! {
        static SWIPL_INITED_THREAD: RefCell<bool> = RefCell::new(false);
    }

    let mut swipl_inited_global = SWIPL_INITED_GLOBAL.lock().unwrap();
    if !*swipl_inited_global {
        let args = [b"swipl\0".as_ptr(), b"-q\0".as_ptr(), null()].as_ptr();
        unsafe { PL_initialise(2, args as *mut *mut i8) };
        *swipl_inited_global = true;
        SWIPL_INITED_THREAD.with(|swipl_inited_thread| {
            *swipl_inited_thread.borrow_mut() = true;
        })
    }

    SWIPL_INITED_THREAD.with(|swipl_inited_thread| {
        let mut swipl_inited_thread = swipl_inited_thread.borrow_mut();
        if !*swipl_inited_thread {
            unsafe { PL_thread_attach_engine(null_mut()) };
            *swipl_inited_thread = true;
        }
    });
}
