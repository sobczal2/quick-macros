use quick_macros::FullCtor;

#[derive(FullCtor)]
struct FieldNamedStruct {
    #[allow(unused)]
    field_1: i32,
}

#[derive(FullCtor)]
pub struct FieldPubNamedStruct {
    #[allow(unused)]
    field_2: i32,
}

#[derive(FullCtor)]
struct PubFieldNamedStruct {
    #[allow(unused)]
    pub field_3: i32,
}

#[derive(FullCtor)]
pub struct PubFieldPubNamedStruct {
    #[allow(unused)]
    pub field_4: i32,
}

#[test]
fn nameof_field_named_struct() {
    let instance = FieldNamedStruct::new(10);
    assert_eq!(instance.field_1, 10);
}

#[test]
fn nameof_field_pub_named_struct() {
    let instance = FieldPubNamedStruct::new(10);
    assert_eq!(instance.field_2, 10);
}

#[test]
fn nameof_pub_field_named_struct() {
    let instance = PubFieldNamedStruct::new(10);
    assert_eq!(instance.field_3, 10);
}

#[test]
fn nameof_pub_field_pub_named_struct() {
    let instance = PubFieldPubNamedStruct::new(10);
    assert_eq!(instance.field_4, 10);
}
