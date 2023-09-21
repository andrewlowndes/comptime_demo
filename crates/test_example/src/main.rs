use comptime::comptime;

#[derive(Debug)]
pub enum TestEnum2 {
    One,
    Two,
    Three,
}

// we can call our library functions here - they will run at compile-time and produce rust code we can use
// the library function must be accessible publically including references to existing data like the type
comptime!(test_lib::into_matching_enum::<test_types::TestEnum>(
    "TestEnum2"
));

fn main() {
    //use the generated From functionality to cast from one enum to another
    let selected: TestEnum2 = test_types::TestEnum::One.into();
    dbg!(selected);
}
