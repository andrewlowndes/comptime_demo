use strum::EnumVariantNames;

// we expose variants names so our lib can iterate over them
#[derive(EnumVariantNames)]
pub enum TestEnum {
    One,
    Two,
    Three,
}
