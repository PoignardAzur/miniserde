use miniserde_derive_venial::Serialize;

#[derive(Serialize)]
enum Enum {
    Variant(i32),
}

fn main() {}
