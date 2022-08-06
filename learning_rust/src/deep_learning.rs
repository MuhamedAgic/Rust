

#[derive(Debug)]
pub struct Node
{
    data: f64
}

impl Node
{
    pub fn new() -> Self
    {
        Self { data: 0.0 }
    }
}


pub mod ActivationFunctions
{
    use crate::deep_learning::Node;

    // Update the values of the nodes in the list
    pub fn relu<T>(layer: &mut [Node]) -> ()
    {
        for i in 0..layer.len()
        {
            if layer[i].data < 0.0
            {
                layer[i].data = 0.0;
            }
        }
    } 
    
    pub fn tanh<T>(layer: &mut [Node]) -> ()
    {
        //layer = layer.iter().map(|node| js_sys::Math::tanh(node.data)).collect();
    } 

    pub fn sigmoid<T>(layer: &mut [Node]) -> ()
    {
        todo!();
    }

    //Probability
    pub fn softmax<T>(layer: &mut [Node]) -> ()
    {
        todo!();
    }

    pub fn binary_step<T>(layer: &mut [Node]) -> ()
    {
        todo!();
    }

}


