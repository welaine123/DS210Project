mod graph;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, StringRecord};
use graph::{Graph, ListOfEdges};

fn process_airports_file(path: &str) -> Result<HashMap<String,String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let header: StringRecord = rdr.headers()?.clone();
    let mut airport_code_to_name = HashMap::new<String, String>;
    for result in rdr.records() {
        match result {
            Ok(record) => {

                let mut names:Vec<String> = vec![];
                for (idx, field) in record.iter().enumerate() {

                    //get the headers
                    let column_name = header.get(idx);

                    if let Some(column_name) = column_name {
                        match column_name {
                            "Name" => names.push(field),
                            "IATA" => names.push(field),
                            _ => {
                                //Columns that are not needed
                                println!("Unknown column: {} - Value: {}", column_name, field);
                            }
                        }
                    } else {
                        println!("Column name is missing for field: {}", field);
                    }
                }
                airport_code_to_name.insert(names[0], names[1]);
            }
            Err(err) => panic!("Error reading CSV record: {}", err),
        }
    }
    Ok(airport_code_to_name)
}

fn process_route_file(path: &str) -> Result<HashSet<(String,String)>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let header: StringRecord = rdr.headers()?.clone();
    let result:HashSet<(String,String)> = HashSet::new();
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let mut names:Vec<String> = vec![];
                for (idx, field) in record.iter().enumerate() {
                    let column_name = header.get(idx);

                    if let Some(column_name) = column_name {
                        match column_name {
                            "Source airport" => names.push(field),
                            "Destination airport" => names.push(field),
                            _ => {
                                println!("Unknown column: {} - Value: {}", column_name, field);
                            }
                        }
                    } else {
                        println!("Column name is missing for field: {}", field);
                    }
                }
                result.insert((names[0],names[1]));
            }
            Err(err) => eprintln!("Error reading CSV record: {}", err),
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    process_airports_file("airports.csv")?;
    process_route_file("routes.csv")?;

    Ok(())
}

