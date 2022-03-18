use miniserde_derive_venial::Serialize;

#[derive(Serialize)]
struct Struct {
    #[serde(skip)]
    x: i32,
}

fn main() {}
