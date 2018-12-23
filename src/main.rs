use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
extern crate nalgebra_glm as glm;
extern crate rand;
use glm::{Vec3};

#[derive(Clone)]
struct DataStructure<'a> {
    id: i32,
    edges: Vec<Edge<'a>>,
    position: Vec3
}

#[derive(Clone)]
struct Edge<'a> {
    attraction: &'a str,
    force: f32
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines = content.split("\n");
    let mut m: HashMap<&str, DataStructure> = HashMap::new();
    let m2: HashMap<&str, DataStructure>;
    let mut index = 0;
    let mut max_force = 0.0f32;
    let mut max_edges = 0;

    for line in lines.skip(1) {
        let v = line.split("|").collect::<Vec<&str>>();
        let prem: &mut DataStructure;

        match m.get_mut(v[0]) {
            Some(p) => prem = p,
            _ => {
                index += 1;

                let random_vector = Vec3::new(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                );

                m.insert(v[0],DataStructure{
                    id: index,
                    edges: Vec::new(),
                    position: random_vector
                });

                prem = m.get_mut(v[0]).unwrap();
            }
        }

        let force = match v[2].parse::<f32>() {
            Ok(v) => v,
            Err(_) => 1.0
        };

        if max_force < force && force < 500.0 {
            max_force = force;
        }

        prem.edges.push(Edge{
            force,
            attraction: v[1]
        });

        if max_edges < prem.edges.len() {
            max_edges += 1;
        }

    }

    m2 = m.clone();


    for _ in 0..10 {
        for (_, value) in m.iter_mut() {
            cluster(value, &m2);
        }
    }

    Ok(())
}

fn cluster(data: &mut DataStructure, map: &HashMap<&str, DataStructure>) {
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);
    let dist = 0.0001f32;

    for edge in data.edges.iter_mut() {
        let attraction = match map.get(edge.attraction) {
            Some(attraction) => attraction,
            None => continue
        };

        if data.id != attraction.id {
            let distance: f32 = glm::distance(&data.position, &attraction.position);
            if distance > dist {
                velocity -= glm::normalize(&(data.position - attraction.position)) * glm::clamp_scalar(distance - dist, 0.0, 100.0) * 0.5 ;
            }
        }
    }

    data.position += velocity;

}
