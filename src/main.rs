use itertools::Itertools;
use std::collections::HashMap;

// Struct to hold possible letters and points combinations
struct Combination {
    first_word_length: i32,
    first_word_points: i32,
    second_word_length: i32,
    second_word_points: i32,
}

// Monster method to find valid higher value letters combinations in the first (longer) word
// Takes the vector of higher value letters and the HashMap of Scrabble points as arguments because fudge
impl Combination {
    fn find_higher_value_letter_combinations(&self, higher_value_letters: &Vec<char>, scrabble_points: &HashMap<char, i32>) {
        // Vector to hold the combinations
        let mut higher_value_letter_combinations: Vec<Vec<&char>> = Vec::new();

        for n in 1..=higher_value_letters.len() {
            // Use Itertools to make combinations of n letters out of the set of higher value letters (0 < n <= number of higher value letters)
            let higher_value_letters_iter = higher_value_letters.iter().combinations(n);
    
            // Put excess points contributions of the higher value letters in a vector and sum them
            for letter_combination in higher_value_letters_iter {
        
                let excess_points_vector: Vec<_> = letter_combination
                .iter()
                .map(|letter| scrabble_points.get(letter).unwrap() - 1)
                .collect();

                let excess_points_sum = excess_points_vector.iter().sum::<i32>();
    
                // Check excess points against points in the first word
                if excess_points_sum == self.first_word_points - self.first_word_length {
                    
                    // Check if letter combination has already been identified (this happens when there are repeats in the 20 letters)
                    let mut no_duplicate = true;
                    
                    for combination in higher_value_letter_combinations.iter() {
                        if combination == &letter_combination {
                            no_duplicate = false;
                        }
                    }

                    // Add new combinations to the vector!
                    if no_duplicate {
                        higher_value_letter_combinations.push(letter_combination);
                    }
                }
            }
        }
    
        // Print the letter combinations for the user
        for combination in higher_value_letter_combinations.iter() {
            println!("{combination:?}");
        }
    }
}

fn main() {

    // Take user input, e.g. letters separated by spaces
    // User input not currently implemented
    let letters = String::from("a a a b c d e e i m m m n n r r s s t y").to_lowercase();

    // Turn input string into a vector of characters
    let letters_vector: Vec<_> = letters.split_whitespace().flat_map(str::chars).collect();

    // Have we been provided 20 characters?
    if letters_vector.len() != 20 {
        panic!("Provide exactly 20 Scrabble letters");
    }

    // Are all characters provided valid Scrabble characters?
    if !letters_vector.iter().all(char::is_ascii_alphabetic) {
        panic!("Provide valid Scrabble characters");
    }

    // Display verified 20 letter input to user
    let mut letter_display = String::with_capacity(20 * std::mem::size_of::<char>());

    for letter in &letters_vector {
        letter_display.push(letter.to_ascii_uppercase());
        letter_display.push(' ');
    }

    println!(
        "
Good morning!

Your 20 letters for today are:
{letter_display}
"
    );

    // Map characters to Scrabble points values
    let scrabble_points: HashMap<char, i32> = HashMap::from([
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
        ('z', 10),
    ]);

    // Create vector of relevant Scrabble points values

    let points_vector: Vec<_> = letters_vector
        .iter()
        .map(|letter| scrabble_points.get(letter).unwrap())
        .collect();
  
    // ...and vector of letters with points value > 1
    
    let mut higher_value_letters = Vec::new();

    for letter in letters_vector.iter() {
        let points = *scrabble_points.get(letter).unwrap();

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
    let points_over_min = points_vector.iter().copied().sum::<i32>() - 20;

    // Take user input of maximum score for the day
    let max_score = 352;

    println!(
        "Today's maximum score is: {max_score}
    
    "
    );

    // Create empty vector to store possible combinations in
    let mut combinations_vector: Vec<Combination> = Vec::new();

    // Find length and points combinations equal to maximum score, by iterating over all possible combinations of word lengths and points
    // This assumes no more than 2 words in the solution
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


    // Display combinations found to user
    println!(
        "Found {} possible combinations today:",
        combinations_vector.len()
    );
    println!();

    for (index, combination) in combinations_vector.iter().enumerate() {
        println!(
            "{} letters worth {} points (scoring {})",
            combination.first_word_length,
            combination.first_word_points,
            combination.first_word_length * combination.first_word_points
        );
        combination.find_higher_value_letter_combinations(&higher_value_letters, &scrabble_points);
        println!("and");
        println!(
            "{} letters worth {} points (scoring {})",
            combination.second_word_length,
            combination.second_word_points,
            combination.second_word_length * combination.second_word_points
        );
        println!();

        if index != combinations_vector.len() - 1 {
            println!("OR");
            println!();
        }
    }
}