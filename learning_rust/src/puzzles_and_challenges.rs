use std::collections::HashSet;
use std::vec::Vec;
use core::str::CharIndices;

// mu puzzle
// Rules and possibilities:
// If a string ends with 'I', 'U' can be added ('MI' can be changed to 'MIU')
// Three 'I's in succession can be changed to a 'U' ('MUIII' can be changed to 'MUU')
// The string 'Mx' (where x is any sequence of letters) can be changed to 'Mxx' ('MUIU' can be changed to 'MUIUUIU')
// Two 'U's in succession can be deleted ('MIUU' can be changed to 'MI')

// CHALLENGE: start with "mi"
// Make transformations such that you end with "mu"

#[derive(Debug)]
pub enum Rules
{
    add_u_after_i = 0,
    change_three_is_to_u = 1,
    duplicate_everything_after_m = 2,
    delete_2_successive_us = 3
}

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
    // Weer x aantal plaatsen terug gaan
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
    let mut chars_copy = element.char_indices(); // Bewust een kopie

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

pub fn check_rules_to_apply(element: &str) -> [bool; 4]
{
    let rules_to_apply = [false, false, false, false];
    // If a string ends with 'I', 'U' can be added ('MI' can be changed to 'MIU')
    if element.ends_with('i')
    {
        //rules_to_apply[Rules::add_u_after_i] = true;
    }

    // Three 'I's in succession can be changed to a 'U' ('MUIII' can be changed to 'MUU')
    let iii_indices: Option<Vec<usize>> = return_occurances_and_indices_of(element, 'i', 3);
    if iii_indices != None
    {
        //rules_to_apply[Rules::change_three_is_to_u] = true;

    }

    // The string 'Mx' (where x is any sequence of letters) can be changed to 'Mxx' ('MUIU' can be changed to 'MUIUUIU')
    let m_indices: Option<Vec<usize>> = return_occurances_and_indices_of(element, 'm', 1);
    if m_indices != None
    {
        //rules_to_apply[Rules::duplicate_everything_after_m] = true;
    }

    // Two 'U's in succession can be deleted ('MIUU' can be changed to 'MI')
    let uu_indices: Option<Vec<usize>> = return_occurances_and_indices_of(element, 'u', 2);
    if uu_indices != None
    {
        //rules_to_apply[Rules::delete_2_successive_us] = true;
    }


    return rules_to_apply;

    todo!();

}

pub fn apply_mi_mu_possibilities(collection: &HashSet<&str>) -> ()
{
    // loop door de lijst
    // voor elk element, check welke regels gelden
    // maak nieuwe elementen adhv die regels (insert de nieuwe)
    let mut rules_to_apply = [false, false, false, false];
    collection.into_iter()
              .for_each(|element| rules_to_apply = check_rules_to_apply(element)); // en ook uitprinten
              
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
    // dat elemant hebben we dan gehad

    while !collection.contains("mu")
    {
        apply_mi_mu_possibilities(&collection);
    }

    todo!();

}