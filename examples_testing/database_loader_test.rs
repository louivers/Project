use crate::{util::loading::read_data, models::query::DataBase};

#[allow(dead_code)]
pub fn loader_test() {
    let beers = read_data(String::from("data/beers.csv"));
    let breweries = read_data(String::from("data/breweries.csv"));
    let categories = read_data(String::from("data/categories.csv"));
    let locations = read_data(String::from("data/locations.csv"));
    let styles = read_data(String::from("data/styles.csv"));
    match (beers, breweries, categories, locations, styles) {
        (Ok(beers), Ok(breweries), Ok(categories), Ok(locations), Ok(styles)) => println!("{}", DataBase::from_record_batches(
            vec![
                beers, 
                breweries, 
                categories, 
                locations,
                styles], 
            vec![String::from("beers"),
                String::from("breweries"),
                String::from("categories"),
                String::from("locations"),
                String::from("styles")])),
        (Err(e), _, _, _, _) => println!("Error reading beers.csv: {}", e),
        (_, Err(e), _, _, _) => println!("Error reading breweries.csv: {}", e),
        (_, _, Err(e), _, _) => println!("Error reading categories.csv: {}", e),
        (_, _, _, Err(e), _) => println!("Error reading locations.csv: {}", e),
        (_, _, _, _, Err(e)) => println!("Error reading styles.csv: {}", e),
    }
}