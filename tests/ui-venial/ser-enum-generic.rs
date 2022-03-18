use miniserde_derive_venial::Serialize;

#[derive(Serialize)]
enum Enum<const T: i32> {
    Variant,
}

fn main() {}
