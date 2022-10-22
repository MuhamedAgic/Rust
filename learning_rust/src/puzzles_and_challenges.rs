use std::collections::HashSet;
use std::vec::Vec;

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

pub fn has_three_successive_is() -> ()
{
    todo!();
}

// Return occurances of a character of an element with its index(es)
pub fn return_occurances_and_indices_of(char_to_find: char, element: &str) -> Option<Vec<usize>>
{
    let mut char_indices = Vec::new();
    let mut chars = element.char_indices();
    chars.into_iter()
         .for_each(|index_char_tuple| if index_char_tuple.1 == char_to_find 
                                                     { 
                                                         
                                                        char_indices.push(index_char_tuple.0.clone())
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

    // The string 'Mx' (where x is any sequence of letters) can be changed to 'Mxx' ('MUIU' can be changed to 'MUIUUIU')

    // Two 'U's in succession can be deleted ('MIUU' can be changed to 'MI')



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