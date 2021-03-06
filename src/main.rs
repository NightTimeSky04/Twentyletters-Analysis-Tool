use chrono::{NaiveTime, Local};
use itertools::Itertools;
use std::collections::HashMap;
use lazy_static::lazy_static;

// Struct to hold possible letters and points combinations
struct Combination {
    first_word_length: i32,
    first_word_points: i32,
    second_word_length: i32,
    second_word_points: i32,
}

// Static HashMap for Scrabble points values
lazy_static!{
    static ref SCRABBLE_POINTS: HashMap<char, i32> = {
        HashMap::from([
            ('a', 1),
            ('b', 3),
            ('c', 3),
            ('d', 2),
            ('e', 1),
            ('f', 4),
            ('g', 2),
            ('h', 4),
            ('i', 1),
            ('j', 8),
            ('k', 5),
            ('l', 1),
            ('m', 3),
            ('n', 1),
            ('o', 1),
            ('p', 3),
            ('q', 10),
            ('r', 1),
            ('s', 1),
            ('t', 1),
            ('u', 1),
            ('v', 4),
            ('w', 4),
            ('x', 8),
            ('y', 4),
            ('z', 10),])
    };
}

// Method to find valid higher value letters combinations in the first (longer) word
// Takes the vector of higher value letters and the HashMap of Scrabble points as arguments
// There has to be a better way!
impl Combination {
    fn find_higher_value_letter_combinations(&self, higher_value_letters: &[char]) -> (Vec<Vec<char>>, Vec<Vec<char>>) {

        // Vectors to hold the combinations
        let mut first_word_letter_combinations: Vec<Vec<char>> = Vec::new();
        let mut second_word_letter_combinations: Vec<Vec<char>> = Vec::new();

        for n in 1..=higher_value_letters.len() {
            // Use Itertools to make combinations of n letters out of the set of higher value letters (0 < n <= number of higher value letters)
            let n_higher_value_letters_iter = higher_value_letters.iter().copied().combinations(n);
    
            // Put excess points contributions of the higher value letters in a vector and sum them
            'n_letter_combinations: for n_letter_combination in n_higher_value_letters_iter {
        
                let excess_points = n_letter_combination
                .iter()
                .map(|letter| SCRABBLE_POINTS.get(letter).unwrap() - 1);

                let excess_points_sum = excess_points.sum::<i32>();
    
                // Check excess points against points in the first word
                if excess_points_sum == self.first_word_points - self.first_word_length {

                    // Check if letter combination has already been identified (this happens when there are repeats in the 20 letters)
                    for combination in first_word_letter_combinations.iter() {
                        if combination == &n_letter_combination {
                            continue 'n_letter_combinations; // skip to next iteration if current combination is a duplicate
                        }
                    }

                    let mut remaining_letters = higher_value_letters.to_vec();

                    'first_word_letters: for first_word_letter in n_letter_combination.iter() {
                        for (index, remaining_letter) in remaining_letters.iter().enumerate() {
                            if first_word_letter == remaining_letter {
                                remaining_letters.remove(index);
                                continue 'first_word_letters
                            }
                        }
                    }
                    
                    first_word_letter_combinations.push(n_letter_combination);
                    second_word_letter_combinations.push(remaining_letters);
                }
            }
        }

        (first_word_letter_combinations, second_word_letter_combinations)
    }
}

fn main() {

    // Take user input, e.g. letters separated by spaces
    // User input not currently implemented
    let letters = String::from("a a c d d e e e g i n n n o r t t u v y").to_lowercase();

    // Turn input string into a vector of characters
    let mut letters_vector: Vec<_> = letters.split_whitespace().flat_map(str::chars).collect();
    letters_vector.sort_unstable();

    // Have we been provided 20 characters?
    if letters_vector.len() != 20 {
        panic!("Provide exactly 20 Scrabble letters");
    }

    // Are all characters provided valid Scrabble characters?
    if !letters_vector.iter().all(char::is_ascii_alphabetic) {
        panic!("Provide valid Scrabble characters");
    }

    // Use local time to display an appropriate greeting
    let current_time: NaiveTime = Local::now().time();

    let end_morning: NaiveTime = NaiveTime::from_hms(12, 0, 0);
    let end_afternoon: NaiveTime = NaiveTime::from_hms(18, 0, 0);

    let mut greeting = String::from("Good evening!");

    if current_time < end_morning {
        greeting = String::from("Good morning!");
    } else if current_time < end_afternoon {
        greeting = String::from("Good afternoon!");
    }

    // Display validated 20 letter input to user
    println!(
        "
{greeting}

Your 20 letters for today are:
{}
", display_letters(&letters_vector)
    );

    // Map characters to Scrabble points values
    // Create vector of relevant Scrabble points values
    let points_vector: Vec<_> = letters_vector
        .iter()
        .map(|letter| SCRABBLE_POINTS.get(letter).unwrap())
        .collect();
  
    // ...and vector of letters with points value > 1
    
    let mut higher_value_letters = Vec::new();

    for letter in letters_vector.iter() {
        let points = *SCRABBLE_POINTS.get(letter).unwrap();

        if points != 1 {
            higher_value_letters.push(*letter);
        }
    }

    /*
    println!(
        "The corresponding Scrabble points values are:
{points_vector:?}
    ");

    println!("The higher value letters in this set are:
{higher_value_letters:?}
    ");
    */

    // Find points over minimum (where minimum is 20, i.e. all letters are worth 1 point)
    let total_points: i32 = points_vector.iter().copied().sum();
    let points_over_min = total_points - 20;

    // Take user input of maximum score for the day
    let max_score = 320;

    println!(
        "Today's maximum score is: {max_score}
    
    "
    );

    // Create empty vector to store possible combinations in
    let mut combinations_vector: Vec<Combination> = Vec::new();

    // Find length and points combinations equal to maximum score, by iterating over all possible combinations of word lengths and points
    // This assumes no more than 2 words in the solution
    if total_points*20 == max_score {
        println!("Today's solution is a single 20 letter word.");
    } else {    
        let first_word_lengths = vec![10, 11, 12, 13, 14, 15, 16, 17, 20];

        for first_word_length in first_word_lengths {
            let second_word_length = 20 - first_word_length;
            let first_word_points_iter = first_word_length..=(first_word_length + points_over_min);

            for first_word_points in first_word_points_iter {
                let second_word_points = 20 + points_over_min - first_word_points;
                let total_score =
                    (first_word_length * first_word_points) + (second_word_length * second_word_points);

                if total_score == max_score {
                    // Store possible maximum score combinations as instances of `Combination` in a vector
                    let possible_combination = Combination {
                        first_word_length,
                        first_word_points,
                        second_word_length,
                        second_word_points,
                    };

                    combinations_vector.push(possible_combination);
                }
            }
        }
    }

    // Display combinations found to user
    let mut combination_s = String::from("combinations");
    if combinations_vector.len() == 1 {
        combination_s = String::from("combination");
    }

    println!(
        "Found {} possible {combination_s} today:",
        combinations_vector.len()
    );
    println!();

    for (index, combination) in combinations_vector.iter().enumerate() {
        let (first_word_letter_combinations, second_word_letter_combinations) = combination.find_higher_value_letter_combinations(&higher_value_letters);

        println!(
            "{} letters worth {} points (scoring {})",
            combination.first_word_length,
            combination.first_word_points,
            combination.first_word_length * combination.first_word_points
        );

        println!("  With the following possible higher value letter combinations:");
        for letter_combination in first_word_letter_combinations.iter() {
            println!("    {}", display_letters(letter_combination));
        }

        println!("and");
        println!(
            "{} letters worth {} points (scoring {})",
            combination.second_word_length,
            combination.second_word_points,
            combination.second_word_length * combination.second_word_points
        );

        println!("  With the following possible higher value letter combinations:");
        for letter_combination in second_word_letter_combinations.iter() {
            println!("    {}", display_letters(letter_combination));
        }

        println!();

        if index != combinations_vector.len() - 1 {
            println!("OR");
            println!();
        }
    }
}

// Function to convert vector of chars to a string of uppercase letters separated by spaces
fn display_letters(letters_vector: &[char]) -> String {
    let mut letter_display = String::with_capacity(20 * std::mem::size_of::<char>());

    for letter in letters_vector {
        letter_display.push(letter.to_ascii_uppercase());
        letter_display.push(' ');
    }

    letter_display
}