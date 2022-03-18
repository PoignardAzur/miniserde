use miniserde_derive_venial::Deserialize;

#[derive(Deserialize)]
enum Enum<const T: i32> {
    Variant,
}

fn main() {}
