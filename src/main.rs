fn main() {

    use std::collections::HashMap;

    // Take user input, e.g. letters separated by spaces
    // User input not currently implemented
    let letters = String::from("a a c d d e e e f f h i i l o p r s t u").to_lowercase();

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

    println!();
    println!("Your 20 letters for today are:");
    println!("{letters_vector:?}");
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

    let mut points_vector = Vec::new();

    for letter in letters_vector.iter() {
        points_vector.push(*scrabble_points.get(letter).unwrap());
    }

    println!("The corresponding Scrabble points values are:");
    println!("{points_vector:?}");
    println!();


    // Find points over minimum (where minimum is all 1s)
    let points_over_min = points_vector.iter().copied().sum::<i32>() - 20;


    // Take user input of maximum score for the day
    let max_score = 353;

    println!("Today's maximum score is: {max_score}");
    println!();


    // Find length and points combinations equal to maximum score
    let first_word_length = vec![10, 11, 12, 13, 14, 15, 16, 17, 20];

    for first_word_length in first_word_length.iter() {
        let second_word_length = 20 - first_word_length;
        let first_word_points_iter = (*first_word_length..(first_word_length + points_over_min + 1));
        for first_word_points in first_word_points_iter {
            let second_word_points = 20 + points_over_min - first_word_points;
            let total_score = (first_word_length * first_word_points) + (second_word_length * second_word_points);
            if total_score == max_score {
                println!("{first_word_length} * {first_word_points} + {second_word_length} * {second_word_points}");
            }
        }
    }

    println!();

}