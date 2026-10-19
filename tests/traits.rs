use simple_impl::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Number(u8);
impl Number {
    newtype_constructor!(pub const fn new(value: u8) -> Self);
    newtype_value!(pub const fn value(self) -> u8);
    newtype_ref!(pub fn inner(&self) -> &u8);
}
impl_from_newtype!(Number, u8);
impl_as_ref_newtype!(Number, u8);
impl_deref_newtype!(Number, u8);
impl_deref_mut_newtype!(Number);

struct Displayed(u8);
impl Displayed { fn value(&self) -> u8 { self.0 } }
impl_display_via!(Displayed, value);

struct Secret(String);
impl_debug_redacted!(Secret, "Secret([REDACTED])");

#[derive(Debug)]
struct TinyError;
impl std::fmt::Display for TinyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("tiny") }
}
impl_std_error!(TinyError);

#[derive(Debug, PartialEq, Eq)]
struct Ordered(u8);
impl Ord for Ordered {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}
impl_partial_ord_via_ord!(Ordered);

#[derive(Debug, PartialEq)]
struct LowError;
#[derive(Debug, PartialEq)]
enum HighError { Low(LowError) }
impl_from_variant!(LowError => HighError, Low);

trait Combine { fn combine(self, rhs: Self) -> Self; }
#[derive(Clone, Copy)]
struct Word(u8);
impl Word { fn combine_impl(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) } }
impl Combine for Word {
    forward_trait_method!(fn combine(self, rhs: Self) -> Self => Word::combine_impl);
}

struct Defaulted(u8);
impl Defaulted { fn new() -> Self { Self(7) } }
impl_default_via_new!(Defaulted);

#[test]
fn trait_helpers_work() {
    let mut n = Number::new(4);
    assert_eq!(n.value(), 4);
    assert_eq!(*n.inner(), 4);
    assert_eq!(*n, 4);
    *n = 9;
    assert_eq!(*n.as_ref(), 9);
    assert_eq!(Number::from(3u8).value(), 3);
    assert_eq!(format!("{}", Displayed(42)), "42");
    assert_eq!(format!("{:?}", Secret("hidden".into())), "Secret([REDACTED])");
    assert_eq!(TinyError.to_string(), "tiny");
    assert!(Ordered(1) < Ordered(2));
    assert_eq!(HighError::from(LowError), HighError::Low(LowError));
    assert_eq!(Word(2).combine(Word(3)).0, 1);
    assert_eq!(Defaulted::default().0, 7);
}
