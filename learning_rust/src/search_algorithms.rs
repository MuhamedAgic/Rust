
// Zoek algoritmen:
// Binary search
// Linear search
// Tree search
// Breadth first search
// Depth first search

extern crate multimap;

use colored::*;
use eframe::epaint::CircleShape;
use epaint::Stroke;
use rand::{Rng};
use core::panic;
use multimap::MultiMap;
use std::{marker::Copy, ops::RangeBounds};
use std::ops::Range;
use eframe::egui;
use egui::RichText;

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

// This debug attribute implements fmt::Debug which will allow us to print the struct using {:?}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct City<'a>
{
    id: u64,
    name: &'a str ,
    popuation: u64,
    area: u64,          //m^2
}

impl<'a> City<'a>
{
    // The new function returns a city (Self)
    pub fn new() -> Self
    {
        City
        {
            id: 0,
            name: "0",
            popuation: 0,
            area: 0
        }
    }
}


// https://stackoverflow.com/questions/62422857/how-to-implement-a-struct-in-rust-that-has-a-list-of-itself-as-a-field
// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}

// Het boeit even niet of de gegevens kloppen...
// Hoeveel connectie combinaties mogelijk voor 10 steden?
// Elke stad verbinden met de andere geeft -> 1 verbonden met 2,3,4,...
// Maar 2 niet weer verbinden met 1, dus het is (n-1)! aantal COMBINATIES (dat is ongelijk aanhet aantal verbindingen per combinatie -> (n-1)+(n-2)...(n-(n-1)))
// Want: 1 verbinden met andere 9, maar dat kan elke 1 zijn die verbind met 9 dus 10 combinaties mogelijk waarbij de eerste verbind met de andere 9
// De 2e die verbind met 8 andere etc
pub fn get_list_of_random_cities() -> [City<'static>; 10]
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
    let mut list_of_random_cities = [City::new(); 10];
    let mut list_of_added_cities_indexes: [i32; 10] = [0; 10];
    for i in 0..10
    {
        list_of_random_cities[i] = 
        {
            City
            {
                id: i as u64, 
                name: list_of_city_names[exclusive_random(0, list_of_city_names.len().try_into().unwrap(), &list_of_added_cities_indexes).unwrap() as usize], 
                popuation: rand::thread_rng().gen_range(10..1000000), 
                area: rand::thread_rng().gen_range(100..5000000)
            }
        };
        list_of_added_cities_indexes[i] = list_of_city_names
                                         .iter()
                                         .position(|&element| element == list_of_random_cities[i].name)
                                         .unwrap() as i32;
    }
    return list_of_random_cities;
}


// Deze functie biedt geen garantie...
pub fn exclusive_random(range_start: i32, range_end: i32, numbers_to_exclude: &[i32]) -> Option<i32>
{
    if numbers_to_exclude.is_empty()
    {
        return Some(rand::thread_rng().gen_range(range_start..range_end));
    }
    
    let mut random_number = rand::thread_rng().gen_range(range_start..range_end);
    let (mut current_try, mut max_num_tries) = (0, 50);

    // Loop door de getallen die je niet wilt genereren, de rest mag wel
    for mut i in 0..numbers_to_exclude.len()
    {            
        if random_number == numbers_to_exclude[i]
        {
            if current_try == max_num_tries
            {
                panic!("Random nummer genereren mislukt! in de range {}..{} mochten de volgende getallen niet voorkomen: {:?}", range_start, range_end, numbers_to_exclude);
            }
            current_try += 1;
            random_number = rand::thread_rng().gen_range(range_start..range_end);
            if i > 0
            {
                i -= 1; // Ik wil hem dan opnieuw checken
            }
            else 
            {
                i = 0;
            }
        }
    }
    
    // Nog een check hoeft niet, of hij paniced, of hij is goed
    return Some(random_number);
}


pub fn return_random_city(list_of_cities: &[City<'static>]) -> Option<City<'static>>
{
    return Some(list_of_cities[rand::thread_rng().gen_range(0..list_of_cities.len())]);
}


pub fn make_random_city_connection(list_of_cities: &[City<'static>]) -> Option<(City<'static>, City<'static>)>
{
    return Some( (return_random_city(list_of_cities).unwrap(), return_random_city(list_of_cities).unwrap()) );
}

// We weten dat combinaties n! is en dat er (n-1)+(n-2)... verbindingen zijn per combinatie
pub fn get_max_possible_city_connections(amount_of_cities: i32) -> i32
{
    // Ik wil niet dat een stad met meer dan een een derde van de andere steden verbonden is
    let mut amount_possible_connections = 0;
    // Hier gebeuren enge dingen...
    // Let wel: 0 tm amountof cities - 1 gebeurt hier, en dat is de bedoeling
    for i in 0..amount_of_cities
    {
        amount_possible_connections += i;
    }
    return amount_possible_connections;
}

pub fn get_amount_of_occurances(city: &City, list_of_city_connections: &MultiMap<City, City>) -> Option<i32>
{
    return Some(list_of_city_connections.get_vec(&city).unwrap().len() as i32);
}

pub fn make_first_city_connection(list_of_cities: &[City<'static>], list_of_city_connections: &mut MultiMap<City, City>) -> ()
{
    let (city1, city2) = make_random_city_connection(list_of_cities).unwrap();
    if list_of_city_connections.is_empty()
    {
        if city1 != city2
        {
            list_of_city_connections.insert(city1, city2);
        }
    }
}

// Adhv een lijst met 10 steden, de steden aan elkaar linken
// TODO: code stijl optimaliseren, code wordt vaak herhaald
// TODO: dubbele bindingen weghalen (of juist laten staan?)
pub fn connect_cities_randomly(list_of_cities: &[City<'static>]) -> Option<MultiMap<City<'static>, City<'static>>>
{
    // Verbind steden op een random manier obv de lijst met steden die je hebt
    let mut city_connections: MultiMap<City, City> = MultiMap::new();

    let max_permissible_city_connections = ((get_max_possible_city_connections(list_of_cities.len() as i32) as f32) / 4.0).round() as i32;
    let max_permissible_connections_per_city = ((get_max_possible_city_connections(list_of_cities.len() as i32) as f32) / 8.0).round() as i32;

    make_first_city_connection(&list_of_cities, &mut city_connections);

    // we maken (max connecties mogelijk / 2) connecties
    'outer: for i in 0..max_permissible_city_connections
    {
        // We kunnen 2 dezelfde steden hebben, dus er moet een check bij komen
        let (mut city1, mut city2) = make_random_city_connection(list_of_cities).unwrap();

        // We proberen maximaal 10 keer om 2 verschillende steden te hebben (moet bijna altijd goed gaan)
        'inner: for j in 0..=9
        {
            if city_connections.contains_key(&city1)
            {
                // Als het max aantal conecties dat een stad heeft gemaakt overschreden zijn...
                let connections_with_city1 = get_amount_of_occurances(&city1, &city_connections).unwrap();

                if connections_with_city1 > max_permissible_connections_per_city
                {
                    // City 1 aanpassen
                    city1 = return_random_city(list_of_cities).unwrap();
                    continue 'inner;
                }
                // Een stad mag niet verbonden zijn met zichzelf
                // Is city1 al verbonden met city2? (Dat mag niet gebeuren)
                // get all values corresponding to key -> unwrap the option -> into iter -> find an item in it which is equal to city2
                let possibly_existing_city_connections = city_connections
                                                                        .get_vec(&city1)
                                                                        .unwrap()
                                                                        .into_iter()
                                                                        .find(|item| item == &&city2);
                match possibly_existing_city_connections
                {
                    // Als die een city2 vind die al verbonden is met city1, dan een nieuwe city2 voor nu
                    Some(&City) => 
                    {
                        city2 = return_random_city(list_of_cities).unwrap();
                        continue 'inner;
                    }
                        // Connectie bestaat niet, city2 is niet verbonden met city1 (want hij is niet gevonden), dus connectie kan gemaakt worden
                    None => 
                    {
                        if city1 != city2
                        {
                            city_connections.insert(city1, city2);
                            continue 'outer;
                        }
                        else 
                        {
                            city2 = return_random_city(list_of_cities).unwrap();
                            continue 'inner;
                        }
                    }
                }
            }
            else
            {
                if j == 9
                {
                    panic!("Na 10x proberen zijn 2 steden nog steeds hetzelfde!");
                }
                if city1 != city2
                {
                    city_connections.insert(city1, city2);
                    continue 'outer;
                }
                // Ze zijn aan elkaar gelijk en een moet aangepast worden
                else 
                {
                    city2 = return_random_city(list_of_cities).unwrap();
                    continue 'inner;
                }
            }
        }
    }

    // Check of connecties zijn gemaakt en return de lijst
    if !city_connections.is_empty()
    {
        return Some(city_connections);
    }
    else
    {
        return None;
    }

}

pub fn print_list_of_cities(list_of_cities: &[City]) -> ()
{
    for i in 0..list_of_cities.len()
    {
        println!("Stad: {:?}", list_of_cities[i]);
    }
}

pub fn print_city_connections(city_connections: &MultiMap<City<'static>, City<'static>>) -> ()
{
    for (key, values) in city_connections.iter_all() 
    {
        println!("Stad: {:?} verbonden met:", key);
        for cities_connected in values
        {
            println!("    {:?}", cities_connected);
        }
    }
}

//Nu kunnen we beginnen met:
// A* search
// Depth first search
// Breadth first search
// Dijkstra's algorithm


// Nice to haves, visualiseren

pub struct MyApp 
{
    name: String,
    age: u32,
}

impl Default for MyApp 
{
    fn default() -> Self 
    {
        Self 
        {
            name: "Muhamed".to_owned(),
            age: 21,
        }
    }
}


struct eguiCircle
{
    shape: epaint::CircleShape
}


impl egui::Widget for &mut eguiCircle
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response 
    { 
        // Teken de cirkel op het scherm
        let circle_shape = egui::Shape::circle_stroke(
            epaint::Pos2{x: self.shape.center.x ,y: self.shape.center.y}, 
            self.shape.radius, 
            self.shape.stroke
        );

        todo!(); //TODO: return response
    }
}


impl eframe::App for MyApp 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.heading("My egui Application");
            ui.horizontal(|ui| 
            {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click each year").clicked() 
            {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // Elke stad is nu een button... Moet nog een cirkel widget maken        

        });
    }
}