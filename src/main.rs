use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


struct DataStructure<'a> {
    id: i32,
    edges: Vec<Edge<'a>>
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
    let mut m: HashMap<String, DataStructure> = HashMap::new();
    let mut index = 0;
    let mut max_force = 0.0f32;
    let mut max_edges = 0;

    for line in lines.skip(1) {
        let v = line.split("|").collect::<Vec<&str>>();
        let prem: &DataStructure;

        match m.get(v[0]) {
            Some(p) => prem = p,
            _ => {
                index += 1;
            }
        }


        println!("{}", index);

    }
    Ok(())
}
