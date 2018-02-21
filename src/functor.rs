use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use swipl_sys::{functor_t, PL_functor_arity_sz, PL_functor_name,
                PL_new_functor_sz};

use atom::Atom;

/// A functor, used to represent a name/arity pair.
#[derive(Clone)]
pub struct Functor {
    pub(crate) inner: functor_t,
}

impl Functor {
    /// Creates a new functor.
    pub fn new(atom: Atom, arity: usize) -> Functor {
        ::init();
        let inner = unsafe { PL_new_functor_sz(atom.inner, arity) };
        Functor { inner }
    }

    /// Creates the functor corresponding to the list constructor.
    pub fn cons() -> Functor {
        Atom::cons().functor(2)
    }

    /// Returns the arity of the functor.
    pub fn arity(&self) -> usize {
        unsafe { PL_functor_arity_sz(self.inner) }
    }

    /// Returns the name of the functor.
    pub fn name(&self) -> Atom {
        let inner = unsafe { PL_functor_name(self.inner) };
        Atom { inner }
    }
}

impl Debug for Functor {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_tuple("Functor")
            .field(&self.name())
            .field(&self.arity())
            .finish()
    }
}

impl Display for Functor {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}/{}", self.name(), self.arity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        panic!("{}", "foo");
        //let foo = Atom::new("foo").functor(1);
        //let bar = Atom::new("bar").functor(2);
        //let baz = Atom::new("baz").functor(0);

        /*
        assert_eq!(foo.to_string(), "foo/1");
        assert_eq!(bar.to_string(), "bar/2");
        assert_eq!(baz.to_string(), "baz/0");
        */
    }
}
