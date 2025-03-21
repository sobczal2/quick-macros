use quick_macros::FieldNames;

#[derive(FieldNames)]
struct FieldNamedStruct {
    #[allow(unused)]
    field_1: i32,
}

#[derive(FieldNames)]
pub struct FieldPubNamedStruct {
    #[allow(unused)]
    field_2: i32,
}

#[derive(FieldNames)]
struct PubFieldNamedStruct {
    #[allow(unused)]
    pub field_3: i32,
}

#[derive(FieldNames)]
pub struct PubFieldPubNamedStruct {
    #[allow(unused)]
    pub field_4: i32,
}

#[test]
fn nameof_field_named_struct() {
    assert_eq!(FieldNamedStruct::nameof_field_1(), "field_1");
}

#[test]
fn nameof_field_pub_named_struct() {
    assert_eq!(FieldPubNamedStruct::nameof_field_2(), "field_2");
}

#[test]
fn nameof_pub_field_named_struct() {
    assert_eq!(PubFieldNamedStruct::nameof_field_3(), "field_3");
}

#[test]
fn nameof_pub_field_pub_named_struct() {
    assert_eq!(PubFieldPubNamedStruct::nameof_field_4(), "field_4");
}
