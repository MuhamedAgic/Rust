use num::pow;

#[derive(Debug, Clone, Copy)]
pub struct Node
{
    pub data: f64
}

impl Node
{
    pub fn new() -> Self
    {
        Self { data: 0.0 }
    }
}

pub mod MathematicalFunctions
{
    use std::f64::consts::*;
    use crate::Node;

    pub fn sigmoid(input: f64) -> f64
    {
        return std::f64::consts::E.powf(input) / (std::f64::consts::E.powf(input) + 1.0);
    }

    pub fn relu(input: f64) -> f64
    {
        if input < 0.0
        {
            return 0.0;
        }
        return input;
    }

    pub fn binary_step(input: f64) -> f64
    {
        if input < 0.0
        {
            return 0.0;
        }
        return 1.0;
    }
}

pub mod ActivationFunctions
{
    use crate::{Node, MathematicalFunctions};

    // inputs: een lijst met nodes (een layer) en een functie (de activatie functie)
    // Nog kijken hoe je dit kan gebruiken
    pub fn apply_activation_function<F>(layer: &mut [Node], function: F) where F: Fn(f64) -> f64 
    {
        layer.iter_mut().for_each(|node| node.data = function(node.data));
    }

    // Update the values of the nodes in the list
    pub fn relu(layer: &mut [Node]) -> ()
    {
        layer.iter_mut().for_each(|node| node.data = MathematicalFunctions::relu(node.data));
    }

    pub fn sigmoid(layer: &mut [Node]) -> ()
    {
        layer.iter_mut().for_each(|node| node.data = MathematicalFunctions::sigmoid(node.data));
    }

    pub fn binary_step(layer: &mut [Node]) -> ()
    {
        layer.iter_mut().for_each(|node| node.data = MathematicalFunctions::binary_step(node.data));
    }
    
    pub fn tanh(layer: &mut [Node]) -> ()
    {
        layer.iter_mut().for_each(|node| node.data = node.data.tanh());
    } 

    //Probability
    pub fn softmax(layer: &mut [Node]) -> ()
    {
        let mut sum_of_e_powered_by_nodes = 0.0;
        for node in layer.iter_mut()
        {
            sum_of_e_powered_by_nodes += std::f64::consts::E.powf(node.data);
        }

        // We hebben de som van alle waarden van de nodes die de macht zijn van het getal e
        // Nu de kans van elke node berekenen
        for node in layer.iter_mut()
        {
            node.data = node.data/ sum_of_e_powered_by_nodes;
        }
    }
}


// Rust essential: https://stackoverflow.com/questions/58254329/how-can-i-change-a-reference-to-an-owned-value-without-clone 

