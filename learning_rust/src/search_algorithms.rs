
// Zoek algoritmen:
// Binary search
// Linear search
// Tree search
// Breadth first search
// Depth first search

use colored::*;
use rand::Rng;

// Helper functie
fn make_even(input_number: f32, add_one: bool) -> f32
{
    if input_number % 2f32 == 0f32
    {
        return input_number;
    }
    else
    {
        let mut output_number = input_number.floor();
        if add_one
        {
            output_number = input_number + 1f32;
        }
        else 
        {
            output_number = input_number - 1f32;
        }
        return output_number;
    }
}

// Voor nu een lijst met getallen doorzoeken
pub fn binary_search(item_to_find: f32, list: &[f32]) -> Option<f32>
{
    println!("Start binary search.");
    println!("Item to find -> {}", &item_to_find);

    // Heeft lijst even of oneven items, deze check hoeft eigenlijk niet, rond maar af naar beneden
    let middle_of_list = (list.len() as f32 / 2.0).floor() as usize;
    let mut current_index = middle_of_list as f32;
    let mut current_item = list[middle_of_list];

    // Bij het midden beginnen, is het groter, dan moeten we de helft van de helft naar achteren
    // Is hij groter dan moet hij de helft van de helft naar voren
    // En zo door tot we het resultaat hebben
    let mut steps_to_move = (middle_of_list as f32 / 2.0).round();

    while current_item != item_to_find
    {
        if steps_to_move == 0.0
        {
            if current_index < list.len() as f32
            {
                steps_to_move = 1.0;
            }
        }

        //println!("Middle of list = {}", &middle_of_list);
        //println!("Current item searched for in list = {}", &current_item);
        //println!("Current index in list = {}", &current_index);
        //println!("Steps to move = {}", &steps_to_move);

        if item_to_find < current_item
        {
            //println!("Item -> {} is KLEINER dan huidige item -> {}", item_to_find, current_item);
            current_item = list[(current_index - steps_to_move) as usize];
            current_index -= steps_to_move;
        }
        else if item_to_find > current_item
        {
            //println!("Item -> {} is GROTER dan huidige item -> {}", item_to_find, current_item);
            current_item = list[(current_index + steps_to_move) as usize];
            current_index += steps_to_move;
        }
        steps_to_move /= 2f32;
        steps_to_move = make_even(steps_to_move.round(), false);
    }
    if item_to_find == current_item
    {
        println!("{} Item om te zoeken -> {}, gevonden -> {}", "GEVONDEN!".green(), item_to_find, current_item);
        return Some(item_to_find);
    }
    else
    {
        println!("{} Item om te zoeken -> {}", "NIET GEVONDEN!".red(), item_to_find);
        None
    }
}


pub fn linear_search(item_to_find: f32, list: &[f32]) -> Option<f32>
{
    println!("Start linear search.");
    println!("Item to find -> {}", &item_to_find);

    // Begin bij het het begin van de lijst    
    // Loop gewoon de lijst af, hebben we een iten dan returnen we hem, zo niet dan niks
    for i in 0..list.len()
    {
        if list[i] == item_to_find
        {
            println!("{} Item om te zoeken -> {}, gevonden -> {}", "GEVONDEN!".green(), item_to_find, list[i]);
            return Some(list[i]);
        }
    }
    println!("{} Item om te zoeken -> {}", "NIET GEVONDEN!".red(), item_to_find);
    None
}

// Tree searches nu
// We maken nodes, wat die voorstellen maakt niet uit
// Die nodes koppelen we aan elkaar, je krijgt dan een boom structuur
// Bedoeling is om in die boom structuur iets te gaan zoeken
// Dit kan op verschillende manieren:

// Tree nodes: https://stackoverflow.com/questions/30262970/array-as-a-struct-field

// Depth first search
// Breadth first search
// A* search

// ideeen:
// - welke stad heeft grootste/kleinste populatie?
// - welke stad heeft grootst/kleinst oppervlak?

// Maak nodes die steden voorstellen, verbind de steden met elkaar, zoek naar een stad adhv zoek algoritmen

// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}
#[derive(Debug, Clone)]
pub struct City
{
    id: u64,
    name: String,
    popuation: u64,
    area: u64,          //m^2
}

#[derive(Debug)]
pub struct CityConnection
{
    city1: City,
    city2: City
}

// https://stackoverflow.com/questions/62422857/how-to-implement-a-struct-in-rust-that-has-a-list-of-itself-as-a-field
// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}

// Het boeit even niet of de gegevnes kloppen...
// Hoeveel connectie combinaties mogelijk voor 10 steden?
// Elke stad verbinden met de andere geeft -> 1 verbonden met 2,3,4,...
// Maar 2 niet weer verbinden met 1, dus het is (n-1)! aantal COMBINATIES (dat is ongelijk aanhet aantal verbindingen -> (n-1)+(n-2)...(n-(n-1)))
// Want: 1 verbinden met andere 9, maar dat kan elke 1 zijn die verbind met 9 dus 10 combinaties
// De 2e die verbind met 8 andere etc
pub fn get_list_of_random_cities() -> [City; 10]
{
    let list_of_city_names = [
        "Amsterdam",
        "Rotterdam",
        "Den Haag",
        "Utrecht",
        "Eindhoven",
        "Arnhem",
        "Almelo",
        "Hengelo",
        "Groningen",
        "Friesland",
        "Lelystad",
        "Groot Ammers",
        "Noord Brabant",
        "Limburg",
        "Maastricht",
        "Haarlem",
        "Gouda",
        "Leiden"
    ];

    // wat irritant is dit...
    let mut list_of_random_cities = [City { id: 1, name: "a".to_string(), popuation: 1, area: 1 }; 10];
    for i in 0..10
    {
        list_of_random_cities[i] = 
        {
            City
            {
                id: i as u64, 
                name: list_of_city_names[rand::thread_rng().gen_range(0..list_of_city_names.len())].to_string(), 
                popuation: rand::thread_rng().gen_range(10..1000000), 
                area: rand::thread_rng().gen_range(100..5000000)
            }
        }
    }
    return list_of_random_cities;
}


pub fn connect_cities_randomly(list_of_cities: &[City; 10]) -> Option<[CityConnection; 20]>
{
    // Verbind steden op een random manier obv de lijst met steden die je hebt
    // We weten dat combinaties (n-1)! is en dat er (n-1)+(n-2)... verbindingen zijn

    for i in 0..19
    {
        // Maak een verbinding, maar als een stad al met een andere verbonden is, die overslaan

    }

    return None;
}
