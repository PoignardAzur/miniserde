use miniserde_derive_venial::Deserialize;

#[derive(Deserialize)]
enum Enum {
    Variant(i32),
}

fn main() {}
