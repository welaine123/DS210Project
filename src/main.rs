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




fn convert_to_numeric(
    airport_data: &HashMap<String, String>,
    route_data: &HashSet<(String, String)>,

) -> (BTreeMap<String, usize>, HashSet<(usize, usize)>) {

    // Create a mapping of airport names to unique numbers
    let mut airport_name_to_number = BTreeMap::new();
    let mut current_number = 0;
    for (code, name) in airport_data.iter() {
        airport_name_to_number
            .entry(code.clone())
            .or_insert_with(|| {
                current_number += 1;
                current_number - 1
            });
        airport_name_to_number
            .entry(name.clone())
            .or_insert_with(|| {
                current_number += 1;
                current_number - 1
            });
    }

    // Convert airport pairs to numeric representation
    let numeric_route_data: HashSet<(usize, usize)> = route_data
        .iter()
        .map(|(source, dest)| {
            (
                *airport_name_to_number.get(source).unwrap(),
                *airport_name_to_number.get(dest).unwrap(),
            )
        })
        .collect();

    (airport_name_to_number, numeric_route_data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let airport_data = process_airports_file("airports.csv")?;
    let route_data = process_route_file("routes.csv")?;

    // Convert airport names to numeric representation
    let (airport_name_to_number, numeric_route_data) =
        convert_to_numeric(&airport_data, &route_data);

    println!("Airport Name to Number: {:?}", airport_name_to_number);
    println!("Numeric Route Data: {:?}", numeric_route_data);

    // Create the graph
    let numeric_edges: graph::ListOfEdges = numeric_route_data.iter().map(|&pair| pair).collect();
    let graph = Graph::create_directed(airport_name_to_number.len(), &numeric_edges);

    // Print the graph
    println!("Graph: {:?}", graph);

    Ok(())
}



// fn main() -> Result<(), Box<dyn Error>> {
//     // read two files
//     let airport_data = process_airports_file("airports.csv")?;
//     let route_data = process_route_file("routes.csv")?;

//     println!("Airport Data: {:?}", airport_data);
//     println!("Route Data: {:?}", route_data);

//     // Convert pairs of routes to numbers
//     let route_data_numbers = convert_to_numbers(&route_data, &airport_data);
//     println!("Route Data Numbers: {:?}", route_data_numbers);

//     // Create graph
//     let route_data_numbers: Vec<_> = route_data_numbers.iter().cloned().collect();
//     let graph = Graph::create_directed(airport_data.len(), &route_data_numbers);


//     Ok(())
// }