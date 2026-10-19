use simple_impl::*;
use std::collections::BTreeMap;

#[test]
fn extraction_and_returns_work() {
    fn parse(v: Option<i32>, r: Result<i32, i32>) -> Result<i32, i32> {
        let a = require_some!(v, 10);
        let b = require_ok!(r, |e| e + 1);
        return_if!(a == 0 => Ok(0));
        return_unless!(b > 0 => Ok(1));
        Ok(a + b)
    }
    assert_eq!(parse(Some(2), Ok(3)), Ok(5));
    assert_eq!(parse(None, Ok(3)), Err(10));
    assert_eq!(parse(Some(2), Err(7)), Err(8));

    fn option(v: Option<i32>) -> i32 { some_or_return!(v => 42) }
    assert_eq!(option(None), 42);
    assert_eq!(option(Some(5)), 5);

    fn result(v: Result<i32, ()>) -> i32 { ok_or_return!(v => 12) }
    assert_eq!(result(Err(())), 12);

    fn nonnegative(v: i32) -> Option<i32> {
        none_if!(v < 0);
        Some(v)
    }
    assert_eq!(nonnegative(-1), None);
}

#[test]
fn loop_flow_works() {
    let mut kept = Vec::new();
    for i in 0..6 {
        continue_if!(i % 2 == 0);
        break_if!(i == 5);
        kept.push(i);
    }
    assert_eq!(kept, [1, 3]);

    let mut options = vec![Some(1), None, Some(2)].into_iter();
    let mut outer_items = 0;
    loop {
        let _next = some_or_break!(options.next());
        outer_items += 1;
    }
    assert_eq!(outer_items, 3);

    let mut i = 0;
    let value = loop {
        i += 1;
        break_if!(i == 3 => i);
    };
    assert_eq!(value, 3);

    let mut sum = 0;
    for v in [Some(1), None, Some(2)] {
        let n = some_or_continue!(v);
        sum += n;
    }
    assert_eq!(sum, 3);
}

#[test]
fn collection_helpers_work() {
    let mut v = vec![1];
    push_if!(v, true => 2);
    push_if!(v, false => 99);
    push_some!(v, Some(3));
    extend_if!(v, true => [4, 5]);
    extend_some!(v, Some([6, 7]));
    push_unique!(v, 7);
    push_unique!(v, 8);
    extend_unique!(v, [8, 9, 9]);
    assert_eq!(v, [1,2,3,4,5,6,7,8,9]);

    let mut map = BTreeMap::new();
    insert_if!(map, true => "a", 1);
    insert_if!(map, false => "b", 2);
    insert_some!(map, "c" => Some(3));
    assert_eq!(map.len(), 2);

    let mut x = 1;
    assign_if!(true; x => 2);
    assign_if_some!(x, Some(3));
    assert_eq!(x, 3);
}
