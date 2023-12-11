//main.rs

mod graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, StringRecord};
use graph::Graph;



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
                                // println!("Unknown column: {} - Value: {}", column_name, field);
                            }
                        }
                    } else {
                        println!("Column name is missing for field: {}", field);
                    }
                }
                airport_code_to_name.insert(names[1].to_string(), names[0].to_string());

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
                                // println!("Unknown column: {} - Value: {}", column_name, field);
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

        // let dst_int = *airport_to_int.entry(dst.clone()).or_insert_with(|| src_int);
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

    
    let routes = match process_route_file("routes.csv") {
        Ok(routes) => routes,
        Err(e) => {
            eprintln!("Failed to process routes file: {}", e);
            return Err(e);
        }
    };

    let airports = match process_airports_file("airports.csv") {
        Ok(airports) => airports,
        Err(e) => {
            eprintln!("Failed to process airports file: {}", e);
            return Err(e);
        }
    };

    // Mapping airport names to integers and create unique pairs
    let (airport_to_int, int_routes) = map_airports_to_integers(&routes);

    let directed_graph = Graph::create_directed(airport_to_int.len(), &int_routes);
    println!("Directed Graph: {:?}", directed_graph);

    let centrality = (&directed_graph).compute_degree_centrality();

    // Mapping integers back to airport names
    let int_to_airport_name = map_integers_to_airport_names(&airport_to_int, &airports);

    // Collecting centrality results into a vector and sort it
    let mut centrality_vec: Vec<(&String, &usize)> = centrality.iter()
    .filter_map(|(int, centrality)| {
        if let Some(name) = int_to_airport_name.get(int) {
            Some((name, centrality))
        } else {
            None // Skip this entry if no name is found
        }
    })
    .collect();

    centrality_vec.sort_by(|a, b| b.1.cmp(a.1));

    // Finally, print the result of top 10 airports that connet most nodes
    println!("Top 10 Airports by Degree Centrality:");
    for (airport, centrality) in centrality_vec.iter().take(10) {
        println!("{}: {}", airport, centrality);

    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_airports_file() -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create("sample_airports.csv")?;
        writeln!(file, "index,Airport ID,Name,City,Country,IATA,ICAO,Latitude,Longitude,Altitude,Timezone,DST,Tz database time zone,Type,Source")?;
        writeln!(file, "0,1,Sample Airport,Sample City,Sample Country,SPL,SPLC,0.0,0.0,0,0,U,UTC,airport,OurAirports")?;
        Ok(())
    }

    #[test]
    fn test_process_airports_file() {
        create_sample_airports_file().unwrap();

        let result = process_airports_file("sample_airports.csv").unwrap();
        assert_eq!(result.get("SPL").unwrap(), "Sample Airport");
    }
}
