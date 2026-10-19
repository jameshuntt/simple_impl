use simple_impl::*;

#[derive(Default)]
struct Inner { qos: u8 }

#[derive(Default)]
struct Example {
    values: Vec<i32>,
    name: String,
    inner: Inner,
    optional: Option<String>,
    enabled: bool,
}

impl Example {
    simple_method!(pub fn doubled_qos(&self) -> u8 => self.inner.qos * 2);
    simple_method!(pub fn reset_name(&mut self) => self.name.clear());

    getter!(pub fn qos(&self) -> u8 => inner.qos);
    getter_ref!(pub fn name(&self) -> &String => name);
    getter_mut!(pub fn name_mut(&mut self) -> &mut String => name);
    getter_option_ref!(pub fn optional(&self) -> Option<&String> => optional);
    getter_option_deref!(pub fn optional_str(&self) -> Option<&str> => optional);
    getter_slice!(pub fn values(&self) -> &[i32] => values);
    len_field!(pub fn len(&self) => values);
    is_empty_field!(pub fn is_empty(&self) => values);
    bool_field!(pub fn enabled(&self) => enabled);
    contains_field!(pub fn contains(&self, value: &i32) => values);
    find_by!(pub fn find(&self, key: &i32) -> Option<&i32> => values, item => *item == key);
    delegate!(pub fn name_len(&self) -> usize => name.len());
    delegate_mut!(pub fn pop(&mut self) -> Option<i32> => values.pop());

    setter!(pub fn set_qos(&mut self, qos: u8) => inner.qos);
    setter_into!(pub fn set_name(&mut self, name: String) => name);
    setter_some!(pub fn set_optional(&mut self, value: String) => optional);
    setter_some_into!(pub fn set_optional_into(&mut self, value: String) => optional);
    clear_collection!(pub fn clear_values(&mut self) => values);
    take_field!(pub fn take_optional(&mut self) -> Option<String> => optional);
    enable_field!(pub fn enable(&mut self) => enabled);
    disable_field!(pub fn disable(&mut self) => enabled);
    toggle_field!(pub fn toggle(&mut self) => enabled);

    with_field!(pub fn with_qos(mut self, qos: u8) -> Self => inner.qos);
    with_into!(pub fn with_name(mut self, name: String) -> Self => name);
    with_some!(pub fn with_optional(mut self, value: String) -> Self => optional);
    with_some_into!(pub fn with_optional_into(mut self, value: String) -> Self => optional);
    with_push!(pub fn with_value(mut self, value: i32) -> Self => values);
}

// Separate type for with_push_into because the field type differs.
#[derive(Default)]
struct Strings { optional_strings: Vec<String> }
impl Strings {
    with_push_into!(pub fn with_string_value(mut self, value: String) -> Self => optional_strings);
    with_extend!(pub fn with_many(mut self, values) -> Self => optional_strings: String);
}

#[derive(Default)]
struct Flags { enabled: bool, roles: Vec<String> }
impl Flags {
    with_flag!(pub fn enabled(mut self) -> Self => enabled);
    without_flag!(pub fn disabled(mut self) -> Self => enabled);
    with_unique_push!(pub fn with_role(mut self, role: String) -> Self => roles);
}

#[test]
fn generated_methods_work() {
    let mut x = Example::default()
        .with_qos(3)
        .with_name("name".to_string())
        .with_optional("value".to_string())
        .with_value(7);
    assert_eq!(x.qos(), 3);
    assert_eq!(x.doubled_qos(), 6);
    assert_eq!(x.name(), "name");
    assert_eq!(x.optional_str(), Some("value"));
    assert_eq!(x.values(), &[7]);
    assert_eq!(x.len(), 1);
    assert!(!x.is_empty());
    assert!(x.contains(&7));
    assert_eq!(x.find(&7), Some(&7));
    assert_eq!(x.name_len(), 4);
    x.name_mut().push('!');
    x.set_qos(4);
    x.set_name("other".to_string());
    x.enable();
    assert!(x.enabled());
    x.toggle();
    assert!(!x.enabled());
    x.disable();
    assert_eq!(x.pop(), Some(7));
    x.clear_values();
    assert!(x.is_empty());
    x.set_optional("a".to_string());
    assert_eq!(x.take_optional().as_deref(), Some("a"));
    x.set_optional_into("b");
    assert_eq!(x.optional_str(), Some("b"));
    x.reset_name();
    assert_eq!(x.name(), "");

    let s = Strings::default().with_string_value("a").with_many(["b".to_string(), "c".to_string()]);
    assert_eq!(s.optional_strings, ["a", "b", "c"]);

    let f = Flags::default().enabled().with_role("ops".into()).with_role("ops".into());
    assert!(f.enabled);
    assert_eq!(f.roles, ["ops"]);
    assert!(!f.disabled().enabled);
}
