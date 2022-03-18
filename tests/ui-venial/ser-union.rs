use miniserde_derive_venial::Serialize;

#[derive(Serialize)]
union Union {
    x: i32,
}

fn main() {}
