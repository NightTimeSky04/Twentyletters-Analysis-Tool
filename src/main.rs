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
    fn find_higher_value_letter_combinations(&self, higher_value_letters: &Vec<&char>, scrabble_points: &HashMap<char, i32>) {
        // Vector to hold the combinations
        let mut higher_value_letter_combinations: Vec<Vec<&&char>> = Vec::new();

        for n in 1..higher_value_letters.len() + 1 {
            // Use Itertools to make combinations of n letters out of the set of higher value letters (0 < n <= number of higher value letters)
            let higher_value_letters_iter = higher_value_letters.iter().combinations(n);
    
            // Put excess points contributions of the higher value letters in a vector and sum them
            for letter_combination in higher_value_letters_iter {
                let mut points_vec = Vec::new();
        
                for letter in letter_combination.iter() {
                    let excess_points = *scrabble_points.get(letter).unwrap() - 1;
                    points_vec.push(excess_points);
                }
        
                let excess_sum = points_vec.iter().sum::<i32>();
    
                // Check excess points against points in the first word
                if excess_sum == self.first_word_points - self.first_word_length {
                    // Check if letter combination has already been identified (this happens when there are repeats in the 20 letters)
                    let mut i = 0;
                    
                    for combination in higher_value_letter_combinations.iter() {
                        if combination == &letter_combination {
                            i += 1
                        }
                    }

                    // Pop new combinations in the vector!
                    if i == 0 {
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
    let letters = String::from("a c d e e f f h h i k l l m o r t u w y").to_lowercase();

    // Turn input string into a vector of characters
    let letters_vector: Vec<_> = letters.split_whitespace().flat_map(str::chars).collect();

    // Have we been provided 20 characters?
    if letters_vector.len() != 20 {
        panic!("Provide exactly 20 Scrabble letters");
    }

    // Are all characters provided valid Scrabble characters?
    for letter in letters_vector.iter() {
        if !letter.is_ascii_alphabetic() {
            panic!("Provide valid Scrabble characters");
        }
    }

    // Display validated 20 letter input to user
    let mut letter_display = String::new();

    for letter in letters_vector.iter() {
        letter_display.push(*letter);
        letter_display.push(' ');
    }
    letter_display = letter_display.to_uppercase();

    println!();
    println!("Good morning!");
    println!();
    println!("Your 20 letters for today are:");
    println!("{letter_display}");
    println!();

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
    let mut points_vector = Vec::new();

    // ...and vector of letters with points value > 1
    let mut higher_value_letters = Vec::new();

    for letter in letters_vector.iter() {
        let points = *scrabble_points.get(letter).unwrap();

        points_vector.push(points);

        if points != 1 {
            higher_value_letters.push(letter);
        }
    }

    /*
    println!("The corresponding Scrabble points values are:");
    println!("{points_vector:?}");
    println!();

    println!("The higher value letters in this set are:");
    println!("{higher_value_letters:?}");
    println!();
    */

    // Find points over minimum (where minimum is 20, i.e. all letters are worth 1 point)
    let points_over_min = points_vector.iter().copied().sum::<i32>() - 20;

    // Take user input of maximum score for the day
    let max_score = 368;

    println!("Today's maximum score is: {max_score}");
    println!();
    println!();

    // Create empty vector to store possible combinations in
    let mut combinations_vector: Vec<Combination> = Vec::new();

    // Find length and points combinations equal to maximum score, by iterating over all possible combinations of word lengths and points
    // This assumes no more than 2 words in the solution
    let first_word_length = vec![10, 11, 12, 13, 14, 15, 16, 17, 20];

    for first_word_length in first_word_length.iter() {
        let second_word_length = 20 - first_word_length; // lengths of words are not independent
        let first_word_points_iter = *first_word_length..(first_word_length + points_over_min + 1);

        for first_word_points in first_word_points_iter {
            let second_word_points = 20 + points_over_min - first_word_points; // points are not independent

            let total_score = (first_word_length * first_word_points) + (second_word_length * second_word_points);

            if total_score == max_score {
                // Store possible maximum score combinations as instances of `Combination` in a vector
                let possible_combination = Combination {
                    first_word_length: *first_word_length,
                    first_word_points,
                    second_word_length,
                    second_word_points,
                };

                combinations_vector.push(possible_combination);
            }
        }
    }

    // Display combinations found to user, alongside higher value letter combinations
    println!("Found {} possible combinations today:", combinations_vector.len());
    println!();

    for combination_with_index in combinations_vector.iter().enumerate() {
        println!("{} letters worth {} points (scoring {})", combination_with_index.1.first_word_length, combination_with_index.1.first_word_points, combination_with_index.1.first_word_length * combination_with_index.1.first_word_points);

        println!("with one of the following combinations of higher value letters:");
        combination_with_index.1.find_higher_value_letter_combinations(&higher_value_letters, &scrabble_points);

        println!("and {} letters worth {} points (scoring {}), with the remaining higher value letters.", combination_with_index.1.second_word_length, combination_with_index.1.second_word_points, combination_with_index.1.second_word_length * combination_with_index.1.second_word_points);
        
        println!();
        if combination_with_index.0 != combinations_vector.len() - 1 {
            println!("OR");
            println!();
        }
    }
}