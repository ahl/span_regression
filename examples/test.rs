use span_regression::{identity, nested};

struct Thing {
    x: i32,
    y: i32,
}

#[identity]
fn doit() -> Thing {
    Thing {
        "howdy".to_string(),
    }
}

#[nested]
fn doit2() -> Thing {
    Thing {
        "yo".to_string(),
    }
}

fn main() {}
