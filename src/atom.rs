use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use swipl_sys::{atom_t, PL_ATOM_dot, PL_ATOM_nil, PL_atom_chars,
                PL_new_atom_mbchars, PL_register_atom, PL_unregister_atom,
                REP_UTF8};

use functor::Functor;

/// An interned string, used for literal atoms and for functors.
pub struct Atom {
    pub(crate) inner: atom_t,
}

impl Atom {
    /// Creates a new Atom.
    pub fn new(s: &str) -> Atom {
        ::init();
        let inner = unsafe {
            PL_new_atom_mbchars(
                REP_UTF8 as i32,
                s.len(),
                s.as_ptr() as *const i8,
            )
        };
        Atom { inner }
    }

    /// Returns the Atom corresponding to the list constructor.
    pub fn cons() -> Atom {
        ::init();
        Atom {
            inner: unsafe { PL_ATOM_dot() },
        }
    }

    /// Returns the Atom corresponding to the empty list.
    pub fn nil() -> Atom {
        ::init();
        Atom {
            inner: unsafe { PL_ATOM_nil() },
        }
    }

    /// Creates a functor from the atom.
    pub fn functor(&self, arity: usize) -> Functor {
        Functor::new(self.clone(), arity)
    }
}

impl AsRef<CStr> for Atom {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(PL_atom_chars(self.inner)) }
    }
}

impl Clone for Atom {
    fn clone(&self) -> Atom {
        unsafe {
            PL_register_atom(self.inner);
        }
        Atom { inner: self.inner }
    }
}

impl Debug for Atom {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_tuple("Atom").field(&self.as_ref()).finish()
    }
}

impl Display for Atom {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Display::fmt(&self.as_ref().to_string_lossy(), fmt)
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe { PL_unregister_atom(self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let foo = Atom::new("foo");
        let bar = Atom::new("bar");
        let baz = Atom::new("baz");

        assert_eq!(format!("{:?}", foo), "Atom(\"foo\")");
        assert_eq!(format!("{:?}", bar), "Atom(\"bar\")");
        assert_eq!(format!("{:?}", baz), "Atom(\"baz\")");
    }

    #[test]
    fn display() {
        use std::io::Write;
        ::std::io::stdout().flush().unwrap();
        let foo = Atom::new("foo");
        let bar = Atom::new("bar");
        let baz = Atom::new("baz");

        assert_eq!(foo.to_string(), "foo");
        assert_eq!(bar.to_string(), "bar");
        assert_eq!(baz.to_string(), "baz");
    }
}
