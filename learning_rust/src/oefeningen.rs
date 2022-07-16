
pub mod oefeningen
{
    // het 3x+1 probleem
    pub fn collatz_conjecture(mut input_number: i64) -> i64
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


    // tel hoe vaak een letter voorkomt in een text bestand
    pub fn count_letter_occurances(path_to_file: &str) -> u64
    {
        use std::fs;

        println!("{}", path_to_file);

        let contents = fs::read_to_string(path_to_file)
                                  .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents);

        34
    }


}