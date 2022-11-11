use std::collections::HashSet;
use std::vec::Vec;
use core::str::CharIndices;
use combinations::Combinations;

// mu puzzle
// Rules and possibilities:
// If a string ends with 'I', 'U' can be added ('MI' can be changed to 'MIU')
// Three 'I's in succession can be changed to a 'U' ('MUIII' can be changed to 'MUU')
// The string 'Mx' (where x is any sequence of letters) can be changed to 'Mxx' ('MUIU' can be changed to 'MUIUUIU')
// Two 'U's in succession can be deleted ('MIUU' can be changed to 'MI')

// CHALLENGE: start with "mi"
// Make transformations such that you end with "mu"

// returns where x successive chars are in a string slice
// for example: get succesive uu's in "miiuiuuiimiuu" would return  ('u',[[5, 6], [11, 12]])
pub fn has_this_amount_successive_chars(chars_in_element: &mut CharIndices, char_to_look_for: &char, x_in_a_row: u32) -> bool
{
    let mut count = 0;
    for i in 0..x_in_a_row
    {
        if chars_in_element.next().unwrap().1 == *char_to_look_for
        {
           count += 1;
        }
    }
    // Weer x aantal plaatsen terug gaan, want we lenen de lijst
    chars_in_element.rev();
    for i in 0..x_in_a_row
    {
        chars_in_element.next();
    }
    // Weer vooruit willen gaan
    chars_in_element.rev();
    // Net zo veel character op een rij gevonden als dat gevonden moest worden
    if count == x_in_a_row
    {
        return true;
    }
    return false;
}

// Return occurances of a character, with its index(es), which occures x times in a row, of an element (string slice)
pub fn return_occurances_and_indices_of(element: &str, char_to_find: char, x_in_a_row: u32) -> Option<Vec<usize>>
{
    let mut char_indices = Vec::new();
    let mut chars = element.char_indices();
    let mut chars_copy = element.char_indices(); // Bewust een kopie van element

    chars.into_iter()
         .for_each(|index_char_tuple| if index_char_tuple.1 == char_to_find 
                                                     { 
                                                         if has_this_amount_successive_chars(&mut chars_copy, &char_to_find, x_in_a_row)
                                                         {
                                                             char_indices.push(index_char_tuple.0.clone())    
                                                         }
                                                     }
                  );

    if char_indices.is_empty()
    {
        return None;
    }
    return Some(char_indices);
}

pub fn get_combination_posibilities(indexes_to_apply_rule: Vec<usize>) -> Vec<Combinations>
{
    // vb -> [  [1], [2], [3], [1, 2], [2, 3], [1, 2, 3]  ]
    let mut all_combinations_to_apply_rule_on_index = Vec<Combinations>::new(); // Lijst met combinaties
    let possible_combinations_of_combinations = indexes_to_apply_rule.len();

    for i in 0..indexes_to_apply_rule
    {
        // we beginnen met [ [1], [2], [3] ], stap 2 wordt [ [1], [2], [3], [1, 2], [2, 3] ], etc...
        let mut possible_indexes_to_apply_rules = Combinations::new(indexes_to_apply_rule, i);
        all_combinations_to_apply_rule_on_index.insert(possible_indexes_to_apply_rules);
    }

    return all_combinations_to_apply_rule_on_index;
}

// Geen zin om iets generieks te maken
pub fn apply_iii_rule_on_element(element: &str, 
                                 collection: &HashSet<&str>, 
                                 combinations_to_make: &Vec<Combinations>) -> ()
{
    if indices_of_rule_to_apply == None
    {
        return;
    }

    // Loop van 1 tm i AANTAL elementen in de combinatie lijst, dus [ [a], [a, b], [a, b, c] ]
    for i in 0..combinations_to_make.unwrap().len()
    {
        // Loop per combinatie lijst en maak de combinaties, dus voor 2 -> [ [a, b], [b, c] ] etc
        for j in 0..combinations_to_make[i].len()
        {
            let mut newElement = ""; 

            // vervang eerste i met een u en verwijder de andere 2 i's
            newElement = element.replace_range(combinations_to_make[i][j]..combinations_to_make[i][j] + 2, "u");
            collection.insert(newElement);
        }
    }
}

pub fn apply_uu_rule_on_element(element: &str, 
                                collection: &HashSet<&str>, 
                                combinations_to_make: &Vec<Combinations>) -> ()
{
    // voor elke index, pas regel toe
    if indices_of_rule_to_apply == None
    {
        return;
    }

    // Loop van 1 tm i AANTAL elementen in de combinatie lijst, dus [ [a], [a, b], [a, b, c] ]
    for i in 0..combinations_to_make.unwrap().len()
    {
        // Loop per combinatie lijst en maak de combinaties, dus voor 2 -> [ [a, b], [b, c] ] etc
        for j in 0..combinations_to_make[i].len()
        {
            let mut newElement = ""; 
            newElement = element.replace_range(combinations_to_make[i][j]..combinations_to_make[i][j] + 1, "");
            collection.insert(newElement);
        }
    }
}

// Niet oneindig grote strings maken
pub fn apply_mx_rule_on_element(element: &str, collection: &HashSet<&str>) -> ()

{
    // voor elke index, pas regel toe
    if indices_of_rule_to_apply == None
    {
        return;
    }

    let mut newElement = ""; 
    for i in 0..combinations_to_make.unwrap().len()
    {
        // voegt zichzelf toe zonder eerste char (next gaat een plek vooruit)
        newElement = element + element.chars().next().as_str();
    }

    collection.insert(newElement);
}

pub fn apply_mi_mu_rules(element: &str, collection: &HashSet<&str>) -> ()
{
    // The string 'Mx' (where x is any sequence of letters) can be changed to 'Mxx' ('MUIU' can be changed to 'MUIUUIU')
    // M heb je alleen aan het begin, dus dit is semi nutteloos

    // We willen niet verdubbelen als er meer dan 2049 characters erin gaan zitten
    // TODO, kijk naar de grootste string, als die groter is dan 1000 niet verdubbelen
    let combinations_to_apply_rule = get_combination_posibilities(m_indices);
    apply_mx_rule_on_element(element, collection);
    
    // If a string ends with 'I', 'U' can be added ('MI' can be changed to 'MIU')
    if element.ends_with('i')
    {
        let newElement = [element, "u"].concat();
        collection.insert(newElement);
    }

    // Three 'I's in succession can be changed to a 'U' ('MUIII' can be changed to 'MUU')
    let iii_indices: Option<Vec<usize>> = return_occurances_and_indices_of(element, 'i', 3);
    if iii_indices != None
    {
        // Get all possible combinations that can be made of element for this rule
        let combinations_to_apply_rule = get_combination_posibilities(iii_indices);
        apply_iii_rule_on_element(element, collection, &combinations_to_apply_rule);
    }


    // Two 'U's in succession can be deleted ('MIUU' can be changed to 'MI')
    let uu_indices: Option<Vec<usize>> = return_occurances_and_indices_of(element, 'u', 2);
    if uu_indices != None
    {
        // Get all possible combinations that can be made of element for this rule
        let combinations_to_apply_rule = get_combination_posibilities(uu_indices);
        apply_uu_rule_on_element(element, collection, &combinations_to_apply_rule);
    }

    // We zijn klaar met het element verwijder hem uit de lijst
}


pub fn apply_mi_mu_possibilities(collection: &HashSet<&str>) -> ()
{
    // loop door de lijst
    // voor elk element, check welke regels gelden
    // maak nieuwe elementen adhv die regels (insert de nieuwe)
    collection.into_iter()
              .for_each(|element| apply_mi_mu_rules(element, collection)); // en ook uitprinten

    // Voeg element toe aan de lijst, verwijder oud element als het geen mu is, TODO
              
    todo!();

}

pub fn mi_to_mu() -> ()
{
    let start = "mi";
    let desired_result = "mu";

    // begin bij mi, kijk welke regels toegepast kunnen worden, pas ze allemaal toe
    // ga dan naar de andere gemaakte strings en kijk welke regels er toegepast kunnen worden, en ga zo door tot je mu hebt
    let mut collection = HashSet::new();
    collection.insert(start);

    // We houden bij op welke strings de regelszijn toegepast zodat we
    // geen onnodig dubbele waardes erin krijgen
    let mut collection_rules_applied: HashSet<&str> = HashSet::new();

    // Zet items in de applied collectie, apply de regels op de lijst
    // De volgende cycle passen we de regels alleen op de nieuw gekomen items
    // Check wel nog even of we het resultaat hebben

    // check voor elk element de regels die toegepast kunnen worden en pas ze toe
    // voeg het item waar je de regels op controlleert en toepast toe aan de rules applied collectie
    // dat elemant hebben we dan gehad, dan kan je hem verwijderen als het geen mu is

    while !collection.contains("mu")
    {
        apply_mi_mu_possibilities(&collection);
    }

    todo!();

}