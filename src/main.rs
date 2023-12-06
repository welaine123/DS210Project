mod graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, StringRecord};
use graph::Graph;
use std::collections::BTreeMap;
// use crate::graph::ListOfEdges


fn process_airports_file(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let header: StringRecord = rdr.headers()?.clone();
    let mut airport_code_to_name = HashMap::<String, String>::new();
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let mut names: Vec<String> = vec![];
                for (idx, field) in record.iter().enumerate() {
                    //get the headers
                    let column_name = header.get(idx);

                    if let Some(column_name) = column_name {
                        match column_name {
                            "Name" => names.push(field.to_string()),
                            "IATA" => names.push(field.to_string()),
                            _ => {
                                //Columns that are not needed
                                println!("Unknown column: {} - Value: {}", column_name, field);
                            }
                        }
                    } else {
                        println!("Column name is missing for field: {}", field);
                    }
                }
                airport_code_to_name.insert(names[0].to_string(), names[1].to_string());
            }
            Err(err) => panic!("Error reading CSV record: {}", err),
        }
    }
    Ok(airport_code_to_name)
}

fn process_route_file(path: &str) -> Result<HashSet<(String, String)>, Box<dyn Error>> {

    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let header: StringRecord = rdr.headers()?.clone();
    let mut result_set: HashSet<(String, String)> = HashSet::new();
    
    for record_result in rdr.records() {
        match record_result {
            Ok(record) => {
                let mut names: Vec<String> = vec![];
                for (idx, field) in record.iter().enumerate() {
                    let column_name = header.get(idx);

                    if let Some(column_name) = column_name {
                        match column_name {
                            "Source airport" => names.push(field.to_string()),
                            "Destination airport" => names.push(field.to_string()),
                            _ => {
                                println!("Unknown column: {} - Value: {}", column_name, field);
                            }
                        }
                    } else {
                        println!("Column name is missing for field: {}", field);
                    }
                }
                result_set.insert((names[0].to_string(), names[1].to_string()));
            }
            Err(err) => panic!("Error reading CSV record: {}", err),
        }
    }
    Ok(result_set)
}


fn map_airports_to_integers(
    routes: &HashSet<(String, String)>,
) -> (HashMap<String, usize>, Vec<(usize, usize)>) {
    let mut airport_to_int = HashMap::new();
    let mut int_routes = Vec::new();
    let mut next_int = 0;

    for (src, dst) in routes {
        let src_int = *airport_to_int.entry(src.clone()).or_insert_with(|| {
            let current = next_int;
            next_int += 1;
            current
        });
        let dst_int = *airport_to_int.entry(dst.clone()).or_insert_with(|| {
            let current = next_int;
            next_int += 1;
            current
        });

        int_routes.push((src_int, dst_int));
    }

    (airport_to_int, int_routes)
}


fn map_integers_to_airport_names(
    airport_to_int: &HashMap<String, usize>,
    airport_code_to_name: &HashMap<String, String>,
) -> HashMap<usize, String> {
    let mut int_to_airport_name = HashMap::new();

    for (code, &int) in airport_to_int {
        if let Some(name) = airport_code_to_name.get(code) {
            int_to_airport_name.insert(int, name.clone());
        }
    }

    int_to_airport_name
}


fn main() -> Result<(), Box<dyn Error>> {

    let routes = process_route_file("routes.csv")?;
    let airport_code_to_name = process_airports_file("airports.csv")?;
    println!("Airport codes to names: {:?}", airport_code_to_name);
    
    let (airport_to_int, int_routes) = map_airports_to_integers(&routes);
    println!("Airport to integer mapping: {:?}", airport_to_int);
    println!("Integer routes: {:?}", int_routes);
    
    let int_to_airport_name = map_integers_to_airport_names(&airport_to_int, &airport_code_to_name);
    println!("Integer to airport name mapping: {:?}", int_to_airport_name);

    // // Create the graph
    // let numeric_edges: graph::ListOfEdges = numeric_route_data.iter().map(|&pair| pair).collect();
    // let graph = Graph::create_directed(airport_name_to_number.len(), &numeric_edges);

    // // Print the graph
    // println!("Graph: {:?}", graph);

    Ok(())
}