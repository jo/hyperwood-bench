use hyperwood::{Model, Point, Slat, Variant, Vector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct BenchParameters {
    width: isize,
    depth: isize,
    height: isize,
}

impl BenchParameters {
    pub fn new(width: isize, depth: isize, height: isize) -> Self {
        assert!(depth % 2 != 0, "Depth must be uneval");
        assert!(depth >= 9, "Depth must be equal or greater than 9");

        Self {
            width,
            depth,
            height,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BenchProperties {
    width: f32,
    depth: f32,
    height: f32,
}

/// Generate a model from given parameters and variant.
pub fn build_model(
    parameters: BenchParameters,
    variant: Variant,
) -> Model<BenchParameters, BenchProperties> {
    let middle = parameters.depth / 2;
    let distance_leg_to_edge = 3;

    let mut slats = vec![];

    // keel
    slats.push(Slat {
        name: "Keel".to_string(),
        layer: middle,
        origin: Point {
            x: distance_leg_to_edge as f32,
            y: middle as f32,
            z: 1.0,
        },
        vector: Vector {
            x: parameters.width as f32 - 2.0 * distance_leg_to_edge as f32,
            y: 0.0,
            z: 0.0,
        },
    });

    // seat
    for y in 0..parameters.depth {
        if y % 2 == 0 {
            slats.push(Slat {
                name: "Seat".to_string(),
                layer: y,
                origin: Point {
                    x: 0.0,
                    y: y as f32,
                    z: parameters.height as f32,
                },
                vector: Vector {
                    x: parameters.width as f32,
                    y: 0.0,
                    z: 0.0,
                },
            });
        }
    }

    // shelf
    for y in 1..parameters.depth - 1 {
        if y % 2 == 0 {
            slats.push(Slat {
                name: "Shelf".to_string(),
                layer: y,
                origin: Point {
                    x: distance_leg_to_edge as f32 - 1.0,
                    y: y as f32,
                    z: 2.0,
                },
                vector: Vector {
                    x: parameters.width as f32 - 2.0 * distance_leg_to_edge as f32 + 2.0,
                    y: 0.0,
                    z: 0.0,
                },
            });
        }
    }

    // legs
    for y in 0..parameters.depth {
        if y % 2 != 0 {
            // outer legs reach the floor
            let z = match y == 1 || y == parameters.depth - 2 {
                true => 0.0,
                // or surround the keel
                false => match y == middle + 1 || y == middle - 1 {
                    true => 1.0,
                    // or otherwise end at the shelf
                    false => 2.0,
                },
            };

            // left
            slats.push(Slat {
                name: "Leg".to_string(),
                layer: y,
                origin: Point {
                    x: distance_leg_to_edge as f32,
                    y: y as f32,
                    z,
                },
                vector: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: parameters.height as f32 - z,
                },
            });

            // right
            slats.push(Slat {
                name: "Leg".to_string(),
                layer: y,
                origin: Point {
                    x: parameters.width as f32 - distance_leg_to_edge as f32,
                    y: y as f32,
                    z,
                },
                vector: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: parameters.height as f32 - z,
                },
            });
        }
    }

    let properties = BenchProperties {
        width: parameters.width as f32 * variant.x,
        depth: parameters.depth as f32 * variant.y,
        height: parameters.height as f32 * variant.z,
    };

    Model {
        parameters,
        properties,
        name: "Bench".to_owned(),
        variant,
        slats,
    }
}
