use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Manifest<Spec> {
    kind: String,
    spec: Spec,
}

#[derive(Deserialize, Serialize)]
struct Service {
    name: String,
    containers: Vec<Container>,
}

#[derive(Deserialize, Serialize)]
struct Container {
    image: String,
}
