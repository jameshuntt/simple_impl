use simple_impl::*;

#[test]
fn pure_checks_work() {
    assert_eq!(count_true!(true, false, true), 2);
    assert!(all!(true, 2 + 2 == 4));
    assert!(any!(false, true));
    assert!(exactly_one!(false, true, false));
    assert!(at_most_one!(false, true, false));
    assert!(implies!(false => false));
    assert!(implies!(true => true));
    assert!(excludes!(true, false));
    assert!(in_range!(3, 1, 4));
    assert!(nonempty!([1, 2]));
    assert!(is_one_of!(2; 1 | 2 | 3));
}

#[test]
fn result_guards_work() {
    fn checked(x: usize, y: usize, values: &[u8]) -> Result<usize, &'static str> {
        ensure!(x > 0, "x");
        ensure_eq!(x, y, "eq");
        ensure_ne!(x, 9, "nine");
        ensure_nonempty!(values, "empty");
        ensure_len!(values, 2, "len");
        ensure_min_len!(values, 1, "min");
        ensure_max_len!(values, 3, "max");
        ensure_range!(x, 1, 8, "range");
        ensure_all!(x > 0 => "x0", y > 0 => "y0");
        ensure_any!("any"; x == 3, y == 4);
        ensure_exactly_one!("one"; x == 3, y == 4);
        ensure_at_most_one!("at-most"; x == 3, y == 4);
        ensure_implies!(x == 3 => y == 3, "implies");
        ensure_excludes!(x == 1, y == 1, "excludes");
        Ok(x)
    }

    assert_eq!(checked(3, 3, &[1, 2]), Ok(3));
    assert_eq!(checked(0, 0, &[1, 2]), Err("x"));
    assert_eq!(checked(3, 4, &[1, 2]), Err("eq"));

    fn empty_only(v: &[u8]) -> Result<(), &'static str> {
        ensure_empty!(v, "not empty");
        Ok(())
    }
    assert_eq!(empty_only(&[]), Ok(()));
    assert_eq!(empty_only(&[1]), Err("not empty"));

    fn fails() -> Result<(), &'static str> { bail!("bad"); }
    assert_eq!(fails(), Err("bad"));
}

#[test]
fn generic_guard_works() {
    fn f(v: i32) -> i32 {
        guard!(v >= 0 => 99);
        v
    }
    assert_eq!(f(-1), 99);
    assert_eq!(f(7), 7);
}

#[test]
fn cleanup_and_whole_method_macros_work() {
    fn guarded(mut v: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        ensure_with!(!v.is_empty(), { v.clear(); }, "empty");
        Ok(v)
    }
    assert_eq!(guarded(vec![1]), Ok(vec![1]));
    assert_eq!(guarded(vec![]), Err("empty"));

    fn fail_after_cleanup(mut v: Vec<u8>) -> Result<(), &'static str> {
        v.push(1);
        bail_with!({ v.clear(); }, "stop");
    }
    assert_eq!(fail_after_cleanup(vec![]), Err("stop"));

    fn generic_guard_with(mut v: Vec<u8>) -> usize {
        guard_with!(!v.is_empty(), { v.clear(); } => 99);
        v.len()
    }
    assert_eq!(generic_guard_with(vec![]), 99);

    #[derive(Debug, PartialEq)]
    struct Checked { n: usize }
    impl Checked {
        checked_constructor!(
            pub fn new(n: usize), &'static str;
            n > 0 => "zero";
            n < 10 => "large";
            => Self { n }
        );

        checked_method!(
            pub fn doubled(&self) -> usize, &'static str;
            self.n < 6 => "too large";
            => self.n * 2
        );
    }
    assert_eq!(Checked::new(3), Ok(Checked { n: 3 }));
    assert_eq!(Checked::new(0), Err("zero"));
    assert_eq!(Checked::new(12), Err("large"));
    assert_eq!(Checked::new(3).unwrap().doubled(), Ok(6));
}
