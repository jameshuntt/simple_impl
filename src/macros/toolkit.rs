//! General-purpose `macro_rules!` helpers for making small Rust implementations declarative.
//!
//! This crate is intentionally dependency-free and standalone. It is a proposed expansion
//! layer for `simple_impl`, not a replacement for the existing `builder_*`, `sc_*`, or derive
//! families.

// -----------------------------------------------------------------------------
// Pure boolean / validation expressions
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! count_true {
    ($($condition:expr),* $(,)?) => {
        0usize $(+ if $condition { 1usize } else { 0usize })*
    };
}

#[macro_export]
macro_rules! all {
    ($($condition:expr),+ $(,)?) => { true $(&& ($condition))+ };
}

#[macro_export]
macro_rules! any {
    ($($condition:expr),+ $(,)?) => { false $(|| ($condition))+ };
}

#[macro_export]
macro_rules! exactly_one {
    ($($condition:expr),+ $(,)?) => { $crate::count_true!($($condition),+) == 1 };
}

#[macro_export]
macro_rules! at_most_one {
    ($($condition:expr),+ $(,)?) => { $crate::count_true!($($condition),+) <= 1 };
}

#[macro_export]
macro_rules! implies {
    ($left:expr => $right:expr $(,)?) => { !($left) || ($right) };
}

#[macro_export]
macro_rules! excludes {
    ($left:expr, $right:expr $(,)?) => { !(($left) && ($right)) };
}

#[macro_export]
macro_rules! in_range {
    ($value:expr, $min:expr, $max:expr $(,)?) => {{
        let value = &$value;
        ($min..=$max).contains(value)
    }};
}

#[macro_export]
macro_rules! nonempty {
    ($value:expr $(,)?) => { !($value).is_empty() };
}

#[macro_export]
macro_rules! is_one_of {
    ($value:expr; $($candidate:pat_param)|+ $(,)?) => {
        ::core::matches!($value, $($candidate)|+)
    };
}

// -----------------------------------------------------------------------------
// Function-body guards and Result validation
// -----------------------------------------------------------------------------

/// Generic guard: return any value when a condition is false.
#[macro_export]
macro_rules! guard {
    ($condition:expr => $return_value:expr $(,)?) => {
        if !($condition) {
            return $return_value;
        }
    };
}

/// Return `Err(error)` when a condition is false.
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr $(,)?) => {
        if !($condition) {
            return ::core::result::Result::Err($error);
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($error:expr $(,)?) => {
        return ::core::result::Result::Err($error)
    };
}

#[macro_export]
macro_rules! guard_with {
    ($condition:expr, $cleanup:block => $return_value:expr $(,)?) => {
        if !($condition) {
            $cleanup
            return $return_value;
        }
    };
}

#[macro_export]
macro_rules! ensure_with {
    ($condition:expr, $cleanup:block, $error:expr $(,)?) => {
        if !($condition) {
            $cleanup
            return ::core::result::Result::Err($error);
        }
    };
}

#[macro_export]
macro_rules! bail_with {
    ($cleanup:block, $error:expr $(,)?) => {{
        $cleanup
        return ::core::result::Result::Err($error);
    }};
}

#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!(($left) == ($right), $error);
    };
}

#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!(($left) != ($right), $error);
    };
}

#[macro_export]
macro_rules! ensure_nonempty {
    ($value:expr, $error:expr $(,)?) => {
        $crate::ensure!(!($value).is_empty(), $error);
    };
}

#[macro_export]
macro_rules! ensure_empty {
    ($value:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).is_empty(), $error);
    };
}

#[macro_export]
macro_rules! ensure_len {
    ($value:expr, $expected:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() == ($expected), $error);
    };
}

#[macro_export]
macro_rules! ensure_min_len {
    ($value:expr, $minimum:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() >= ($minimum), $error);
    };
}

#[macro_export]
macro_rules! ensure_max_len {
    ($value:expr, $maximum:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() <= ($maximum), $error);
    };
}

#[macro_export]
macro_rules! ensure_range {
    ($value:expr, $minimum:expr, $maximum:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::in_range!($value, $minimum, $maximum), $error);
    };
}

#[macro_export]
macro_rules! ensure_all {
    ($($condition:expr => $error:expr),+ $(,)?) => {
        $( $crate::ensure!($condition, $error); )+
    };
}

#[macro_export]
macro_rules! ensure_any {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::any!($($condition),+), $error);
    };
}

#[macro_export]
macro_rules! ensure_exactly_one {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::exactly_one!($($condition),+), $error);
    };
}

#[macro_export]
macro_rules! ensure_at_most_one {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::at_most_one!($($condition),+), $error);
    };
}

#[macro_export]
macro_rules! ensure_implies {
    ($left:expr => $right:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::implies!($left => $right), $error);
    };
}

#[macro_export]
macro_rules! ensure_excludes {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::excludes!($left, $right), $error);
    };
}

// -----------------------------------------------------------------------------
// Extraction and early control flow
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! require_some {
    ($option:expr, $error:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return ::core::result::Result::Err($error),
        }
    };
}

#[macro_export]
macro_rules! require_ok {
    ($result:expr, $map_error:expr $(,)?) => {
        match $result {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(error) => {
                return ::core::result::Result::Err(($map_error)(error));
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return {
    ($option:expr => $return_value:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return $return_value,
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ($result:expr => $return_value:expr $(,)?) => {
        match $result {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(_) => return $return_value,
        }
    };
}

#[macro_export]
macro_rules! return_if {
    ($condition:expr => $return_value:expr $(,)?) => {
        if $condition {
            return $return_value;
        }
    };
}

#[macro_export]
macro_rules! return_unless {
    ($condition:expr => $return_value:expr $(,)?) => {
        if !($condition) {
            return $return_value;
        }
    };
}

#[macro_export]
macro_rules! none_if {
    ($condition:expr $(,)?) => {
        if $condition {
            return ::core::option::Option::None;
        }
    };
}

#[macro_export]
macro_rules! break_if {
    ($condition:expr $(,)?) => {
        if $condition {
            break;
        }
    };
    ($condition:expr => $value:expr $(,)?) => {
        if $condition {
            break $value;
        }
    };
}

#[macro_export]
macro_rules! continue_if {
    ($condition:expr $(,)?) => {
        if $condition {
            continue;
        }
    };
}

#[macro_export]
macro_rules! some_or_continue {
    ($option:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => continue,
        }
    };
}

#[macro_export]
macro_rules! some_or_break {
    ($option:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => break,
        }
    };
}

// -----------------------------------------------------------------------------
// Collections and conditional mutation
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! push_if {
    ($collection:expr, $condition:expr => $value:expr $(,)?) => {
        if $condition {
            ($collection).push($value);
        }
    };
}

#[macro_export]
macro_rules! push_some {
    ($collection:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            ($collection).push(value);
        }
    };
}

#[macro_export]
macro_rules! extend_if {
    ($collection:expr, $condition:expr => $values:expr $(,)?) => {
        if $condition {
            ($collection).extend($values);
        }
    };
}

#[macro_export]
macro_rules! extend_some {
    ($collection:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(values) = $option {
            ($collection).extend(values);
        }
    };
}

#[macro_export]
macro_rules! insert_if {
    ($map:expr, $condition:expr => $key:expr, $value:expr $(,)?) => {
        if $condition {
            ($map).insert($key, $value);
        }
    };
}

#[macro_export]
macro_rules! insert_some {
    ($map:expr, $key:expr => $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            ($map).insert($key, value);
        }
    };
}

#[macro_export]
macro_rules! assign_if {
    ($condition:expr; $place:expr => $value:expr $(,)?) => {
        if $condition {
            $place = $value;
        }
    };
}

#[macro_export]
macro_rules! assign_if_some {
    ($place:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            $place = value;
        }
    };
}

#[macro_export]
macro_rules! push_unique {
    ($collection:expr, $value:expr $(,)?) => {{
        let collection = &mut $collection;
        let value = $value;
        if !collection.iter().any(|existing| existing == &value) {
            collection.push(value);
        }
    }};
}

#[macro_export]
macro_rules! extend_unique {
    ($collection:expr, $values:expr $(,)?) => {{
        for value in $values {
            $crate::push_unique!($collection, value);
        }
    }};
}

// -----------------------------------------------------------------------------
// Generic one-expression methods: the catch-all for trivial impl bodies
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! simple_method {
    ($(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) -> $ret:ty => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> $ret {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name($($args)*) {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($($args:tt)*) -> $ret:ty => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($($args)*) -> $ret {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($($args:tt)*) => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($($args)*) {
            $body
        }
    };
}

/// Define a whole Result-returning method as ordered checks plus a final value.
#[macro_export]
macro_rules! checked_method {
    (
        $(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) -> $ok:ty, $error_ty:ty;
        $($condition:expr => $error:expr;)*
        => $value:expr $(;)?
    ) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> ::core::result::Result<$ok, $error_ty> {
            $( $crate::ensure!($condition, $error); )*
            ::core::result::Result::Ok($value)
        }
    };
}

/// Constructor-shaped version of `checked_method!`: successful output is `Self`.
#[macro_export]
macro_rules! checked_constructor {
    (
        $(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*), $error_ty:ty;
        $($condition:expr => $error:expr;)*
        => $value:expr $(;)?
    ) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> ::core::result::Result<Self, $error_ty> {
            $( $crate::ensure!($condition, $error); )*
            ::core::result::Result::Ok($value)
        }
    };
}

// -----------------------------------------------------------------------------
// Accessor / delegation method generators
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! getter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> $ret {
            self.$first$(.$rest)*
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(&self) -> $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(&self) -> $ret {
            self.$first$(.$rest)*
        }
    };
}

#[macro_export]
macro_rules! getter_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &$ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &$ret {
            &self.$first$(.$rest)*
        }
    };
}

#[macro_export]
macro_rules! getter_mut {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> &mut $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> &mut $ret {
            &mut self.$first$(.$rest)*
        }
    };
}

#[macro_export]
macro_rules! getter_option_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> Option<&$ret:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> ::core::option::Option<&$ret> {
            self.$field.as_ref()
        }
    };
}

#[macro_export]
macro_rules! getter_option_deref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> Option<&$ret:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> ::core::option::Option<&$ret> {
            self.$field.as_deref()
        }
    };
}

#[macro_export]
macro_rules! getter_slice {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &[$elem:ty] => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &[$elem] {
            &self.$field
        }
    };
}

#[macro_export]
macro_rules! len_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> usize {
            self.$field.len()
        }
    };
}

#[macro_export]
macro_rules! is_empty_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> bool {
            self.$field.is_empty()
        }
    };
}

#[macro_export]
macro_rules! bool_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> bool {
            self.$field
        }
    };
}

#[macro_export]
macro_rules! contains_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $ty) -> bool {
            self.$field.contains($arg)
        }
    };
}

#[macro_export]
macro_rules! find_by {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $arg_ty:ty) -> Option<&$item_ty:ty> => $field:ident, $item:ident => $condition:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $arg_ty) -> ::core::option::Option<&$item_ty> {
            self.$field.iter().find(|$item| $condition)
        }
    };
}

#[macro_export]
macro_rules! delegate {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> $ret:ty => $field:ident.$method:ident() $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> $ret {
            self.$field.$method()
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $ty:ty) -> $ret:ty => $field:ident.$method:ident($pass:ident) $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $ty) -> $ret {
            self.$field.$method($pass)
        }
    };
}

#[macro_export]
macro_rules! delegate_mut {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> $ret:ty => $field:ident.$method:ident() $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> $ret {
            self.$field.$method()
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) -> $ret:ty => $field:ident.$method:ident($pass:ident) $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) -> $ret {
            self.$field.$method($pass)
        }
    };
}

#[macro_export]
macro_rules! newtype_value {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(self) -> $ret:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name(self) -> $ret { self.0 }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(self) -> $ret:ty $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(self) -> $ret { self.0 }
    };
}

#[macro_export]
macro_rules! newtype_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &$ret:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &$ret { &self.0 }
    };
}

// -----------------------------------------------------------------------------
// Setters and consuming builders
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! setter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) {
            self.$first$(.$rest)* = $arg;
        }
    };
}

#[macro_export]
macro_rules! setter_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: impl ::core::convert::Into<$ty>) {
            self.$first$(.$rest)* = $arg.into();
        }
    };
}

#[macro_export]
macro_rules! setter_some {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) {
            self.$field = ::core::option::Option::Some($arg);
        }
    };
}

#[macro_export]
macro_rules! setter_some_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: impl ::core::convert::Into<$ty>) {
            self.$field = ::core::option::Option::Some($arg.into());
        }
    };
}

#[macro_export]
macro_rules! clear_collection {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field.clear();
        }
    };
}

#[macro_export]
macro_rules! take_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> Option<$ty:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> ::core::option::Option<$ty> {
            self.$field.take()
        }
    };
}

#[macro_export]
macro_rules! enable_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = true;
        }
    };
}

#[macro_export]
macro_rules! disable_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = false;
        }
    };
}

#[macro_export]
macro_rules! toggle_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = !self.$field;
        }
    };
}

#[macro_export]
macro_rules! with_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$first$(.$rest)* = $arg;
            self
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(mut self, $arg: $ty) -> Self {
            self.$first$(.$rest)* = $arg;
            self
        }
    };
}

#[macro_export]
macro_rules! with_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field = $arg.into();
            self
        }
    };
}

#[macro_export]
macro_rules! with_some {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$field = ::core::option::Option::Some($arg);
            self
        }
    };
}

#[macro_export]
macro_rules! with_some_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field = ::core::option::Option::Some($arg.into());
            self
        }
    };
}

#[macro_export]
macro_rules! with_push {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$field.push($arg);
            self
        }
    };
}

#[macro_export]
macro_rules! with_push_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field.push($arg.into());
            self
        }
    };
}

#[macro_export]
macro_rules! with_extend {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident) -> Self => $field:ident: $elem:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name<I>(mut self, $arg: I) -> Self
        where
            I: ::core::iter::IntoIterator<Item = $elem>,
        {
            self.$field.extend($arg);
            self
        }
    };
}

#[macro_export]
macro_rules! with_flag {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self) -> Self {
            self.$field = true;
            self
        }
    };
}

#[macro_export]
macro_rules! without_flag {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self) -> Self {
            self.$field = false;
            self
        }
    };
}

#[macro_export]
macro_rules! with_unique_push {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            $crate::push_unique!(self.$field, $arg);
            self
        }
    };
}

// -----------------------------------------------------------------------------
// Constructors and trait boilerplate
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! newtype_constructor {
    ($(#[$meta:meta])* $vis:vis fn $name:ident($arg:ident: $ty:ty) -> Self $(;)?) => {
        $(#[$meta])*
        $vis fn $name($arg: $ty) -> Self { Self($arg) }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($arg:ident: $ty:ty) -> Self $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($arg: $ty) -> Self { Self($arg) }
    };
}

#[macro_export]
macro_rules! impl_default_via_new {
    ($ty:ty $(;)?) => {
        impl ::core::default::Default for $ty {
            fn default() -> Self { Self::new() }
        }
    };
}

#[macro_export]
macro_rules! impl_from_newtype {
    ($wrapper:ty, $inner:ty $(;)?) => {
        impl ::core::convert::From<$inner> for $wrapper {
            fn from(value: $inner) -> Self { Self(value) }
        }
    };
}

#[macro_export]
macro_rules! impl_as_ref_newtype {
    ($wrapper:ty, $inner:ty $(;)?) => {
        impl ::core::convert::AsRef<$inner> for $wrapper {
            fn as_ref(&self) -> &$inner { &self.0 }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_newtype {
    ($wrapper:ty, $target:ty $(;)?) => {
        impl ::core::ops::Deref for $wrapper {
            type Target = $target;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut_newtype {
    ($wrapper:ty $(;)?) => {
        impl ::core::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    };
}

#[macro_export]
macro_rules! impl_display_via {
    ($ty:ty, $method:ident $(;)?) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.$method())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_debug_redacted {
    ($ty:ty, $text:expr $(;)?) => {
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str($text)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_std_error {
    ($ty:ty $(;)?) => {
        impl ::std::error::Error for $ty {}
    };
}

#[macro_export]
macro_rules! impl_partial_ord_via_ord {
    ($ty:ty $(;)?) => {
        impl ::core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(self.cmp(other))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_variant {
    ($source:ty => $target:ty, $variant:ident $(;)?) => {
        impl ::core::convert::From<$source> for $target {
            fn from(error: $source) -> Self {
                Self::$variant(error)
            }
        }
    };
}

#[macro_export]
macro_rules! forward_trait_method {
    ($(#[$meta:meta])* fn $name:ident(self) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(self) -> $ret { $target(self) }
    };
    ($(#[$meta:meta])* fn $name:ident(self, $arg:ident: $ty:ty) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(self, $arg: $ty) -> $ret { $target(self, $arg) }
    };
    ($(#[$meta:meta])* fn $name:ident(&self) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(&self) -> $ret { $target(self) }
    };
    ($(#[$meta:meta])* fn $name:ident(&self, $arg:ident: $ty:ty) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(&self, $arg: $ty) -> $ret { $target(self, $arg) }
    };
}
