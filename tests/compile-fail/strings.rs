#![feature(plugin)]
#![plugin(clippy)]

#[deny(string_add)]
#[allow(string_add_assign)]
fn add_only() { // ignores assignment distinction
    let mut x = "".to_owned();

    for _ in 1..3 {
        x = x + "."; //~ERROR you added something to a string.
    }

    let y = "".to_owned();
    let z = y + "..."; //~ERROR you added something to a string.

    assert_eq!(&x, &z);
}

#[deny(string_add_assign)]
fn add_assign_only() {
    let mut x = "".to_owned();

    for _ in 1..3 {
        x = x + "."; //~ERROR you assigned the result of adding something to this string.
    }

    let y = "".to_owned();
    let z = y + "...";

    assert_eq!(&x, &z);
}

#[deny(string_add, string_add_assign)]
fn both() {
    let mut x = "".to_owned();

    for _ in 1..3 {
        x = x + "."; //~ERROR you assigned the result of adding something to this string.
    }

    let y = "".to_owned();
    let z = y + "..."; //~ERROR you added something to a string.

    assert_eq!(&x, &z);
}

#[allow(dead_code, unused_variables)]
#[deny(string_lit_as_bytes)]
fn str_lit_as_bytes() {
    let bs = "hello there".as_bytes();
    //~^ERROR calling `as_bytes()`
    //~|HELP byte string literal
    //~|SUGGESTION b"hello there"

    // no warning, because this cannot be written as a byte string literal:
    let ubs = "☃".as_bytes();

    let strify = stringify!(foobar).as_bytes();
}

#[allow(dead_code, unused_variables)]
#[deny(string_as_str)]
fn str_as_str() {
    let bs = String::from("hello there");
    let slice = &["1".to_string(), "2".to_string(), "3".to_string()];

    let c = bs.as_str().chars().count();
    //~^ERROR calling `as_str()`
    //~|HELP `&` syntax
    //~|SUGGESTION &bs

    // no warning, because this cannot be written as a byte string literal:
    let _ = slice.iter().any(|v| v.as_str() == "1");
    let _ = slice.iter().map(|v| v.as_str().to_uppercase());
}

fn main() {
    add_only();
    add_assign_only();
    both();

    // the add is only caught for `String`
    let mut x = 1;
    ; x = x + 1;
    //~^ WARN assign_op_pattern
    //~| HELP replace
    //~| SUGGESTION ; x += 1;
    assert_eq!(2, x);

    str_lit_as_bytes();
    str_as_str();
}
