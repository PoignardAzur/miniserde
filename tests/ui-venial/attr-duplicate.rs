use miniserde_derive_venial::Serialize;

// FIXME - handle attributes with multiple items

#[derive(Serialize)]
struct Struct {
    #[serde(rename = "A")]
    #[serde(rename = "B")]
    x: i32,
}

fn main() {}
