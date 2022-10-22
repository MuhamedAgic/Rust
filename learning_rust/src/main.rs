mod search_algorithms;
mod deep_learning;
mod egui;
mod puzzles_and_challenges;

pub use search_algorithms::*;
pub use deep_learning::*;
pub use crate::egui::*;
pub use puzzles_and_challenges::*;

use rand::{Rng};

fn main() 
{
    println!("Hello, world!");

    let list = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0];

    for i in 1..20
    {
        let _result_binary_search = search_algorithms::binary_search(i as f32, &list);
        let _result_linear_search = search_algorithms::linear_search(i as f32, &list);
    }

    // Cities
    let list_of_cities = get_list_of_random_cities();
    print_list_of_cities(&list_of_cities);

    let city_connections = connect_cities_randomly(&list_of_cities);

    print_city_connections(&city_connections.unwrap());

    // Neural network
    let mut input_layer = [Node::new(); 10];
    input_layer.iter_mut().for_each(|node| node.data = rand::thread_rng().gen_range(-5..10) as f64); 

    println!("Input layer: {:?}", input_layer);
    
    ActivationFunctions::binary_step(&mut input_layer);

    println!("Input layer: {:?}", input_layer);

    // TODO, gewichten en biasen toevoegen, en de nodes aan elkaar verbinden!



    // GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(egui::MyApp::default())),
    );

}



