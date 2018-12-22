use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
extern crate nalgebra;
extern crate rand;
use nalgebra::{ Vector3};

struct DataStructure<'a> {
    id: i32,
    edges: Vec<Edge<'a>>,
    position: Vector3<f32>
}

struct Edge<'a> {
    attraction: &'a str,
    force: f32
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);

    let mut file = File::open(&args[1])?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines = content.split("\n");
    let mut m: HashMap<&str, DataStructure> = HashMap::new();
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

                let randomVector = Vector3::new(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                );

                m.insert(v[0],DataStructure{
                    id: index,
                    edges: Vec::new(),
                    position: randomVector
                });

                prem = m.get_mut(v[0]).unwrap();
            }
        }

        let force = match v[2].parse::<f32>() {
            Ok(v) => v,
            Err(_) => 1.0
        };



        prem.edges.push(Edge{
            force: force,
            attraction: v[1]
        });
        println!("{}", index);

    }
    Ok(())
}
