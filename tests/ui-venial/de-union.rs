use miniserde_derive_venial::Deserialize;

#[derive(Deserialize)]
union Union {
    x: i32,
}

fn main() {}
