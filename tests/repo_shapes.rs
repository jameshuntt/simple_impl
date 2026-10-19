use simple_impl::*;

// Shapes taken from recurring patterns in the user's repositories.

#[derive(Default)]
struct Policy {
    requires_sudo: bool,
    iam_scope: Option<String>,
    path_roles: Vec<String>,
}
impl Policy {
    with_field!(pub fn with_requires_sudo(mut self, value: bool) -> Self => requires_sudo);
    with_some_into!(pub fn with_iam_scope(mut self, value: String) -> Self => iam_scope);
    with_unique_push!(pub fn with_path_role(mut self, role: String) -> Self => path_roles);
    bool_field!(pub fn requires_sudo(&self) => requires_sudo);
    getter_option_deref!(pub fn iam_scope(&self) -> Option<&str> => iam_scope);
    getter_slice!(pub fn path_roles(&self) -> &[String] => path_roles);
}

struct RawBufShape { len: usize, cap: usize }
impl RawBufShape {
    getter!(pub fn len(&self) -> usize => len);
    getter!(pub fn capacity(&self) -> usize => cap);
    simple_method!(pub fn is_empty(&self) -> bool => self.len == 0);
}

struct Participant { id: String, role: u8 }
impl Participant {
    simple_method!(pub fn id(&self) -> &str => &self.id);
    getter!(pub fn role(&self) -> u8 => role);
}

#[test]
fn repository_shapes_compile_and_behave() {
    let p = Policy::default()
        .with_requires_sudo(true)
        .with_iam_scope("ops:deploy")
        .with_path_role("deployer".to_string())
        .with_path_role("deployer".to_string());
    assert!(p.requires_sudo());
    assert_eq!(p.iam_scope(), Some("ops:deploy"));
    assert_eq!(p.path_roles(), &["deployer".to_string()]);

    let r = RawBufShape { len: 0, cap: 4 };
    assert!(r.is_empty());
    assert_eq!(r.capacity(), 4);

    let who = Participant { id: "node-a".into(), role: 2 };
    assert_eq!(who.id(), "node-a");
    assert_eq!(who.role(), 2);
}
