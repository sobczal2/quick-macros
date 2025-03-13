#![cfg(feature = "derive")]

use quick_macros::FieldNames;

#[derive(FieldNames)]
struct TestFieldStruct {
    #[allow(unused)]
    test_field_1: i32,
}

#[derive(FieldNames)]
pub struct TestFieldPubStruct {
    #[allow(unused)]
    test_field_2: i32,
}

#[derive(FieldNames)]
struct TestPubFieldStruct {
    #[allow(unused)]
    pub test_field_3: i32,
}

#[derive(FieldNames)]
pub struct TestPubFieldPubStruct {
    #[allow(unused)]
    pub test_field_4: i32,
}

#[test]
fn nameof_field_struct() {
    assert_eq!(TestFieldStruct::nameof_test_field_1(), "test_field_1");
}

#[test]
fn nameof_field_pub_struct() {
    assert_eq!(TestFieldPubStruct::nameof_test_field_2(), "test_field_2");
}

#[test]
fn nameof_pub_field_struct() {
    assert_eq!(TestPubFieldStruct::nameof_test_field_3(), "test_field_3");
}

#[test]
fn nameof_pub_field_pub_struct() {
    assert_eq!(TestPubFieldPubStruct::nameof_test_field_4(), "test_field_4");
}
