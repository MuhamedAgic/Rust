mod oefeningen;
mod search_algorithms;

pub use oefeningen::*;
pub use search_algorithms::*;





fn main() 
{
    println!("Hello, world!");

    let _x = oefeningen::oefeningen::collatz_conjecture(27);

    // let _y = oefeningen::oefeningen::count_letter_occurances("..\\test.txt");

    let list = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0];

    //let result = search_algorithms::binary_search(17.0, &list);

    for i in 1..20
    {
        let result_binary_search = search_algorithms::binary_search(i as f32, &list);
        let result_linear_search = search_algorithms::linear_search(i as f32, &list);
    }
    

}



