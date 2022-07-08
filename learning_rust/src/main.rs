

// het 3x+1 probleem
fn collatz_conjecture(mut input_number: i64) -> i64
{
    for i in 0..std::i64::MAX
    {
        println!("huidig nummer => {}", input_number);
        if input_number == 1
        {
            println!("We zijn tot {} gekomen na {} stappen!", input_number, i);
            return i;
        }
        else if input_number % 2 == 0
        {
            input_number /= 2;
        }
        else if input_number % 2 != 0
        {
            input_number = input_number * 3 + 1;
        }
    }
    println!("Bereik te groot!");
    return 0;
}


fn main() {
    println!("Hello, world!");

    let _x = collatz_conjecture(27);

}



