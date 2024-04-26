use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("What kind of puzzle?");
        println!("1: Button");
        println!("2: Password");
        println!("3: Complicated Wires");

        let Ok(_) = io::stdin().read_line(&mut input) else { return };

        println!();

        match input.as_str().trim() {
            "1" => process_button(),
            "2" => process_password(),
            "3" => process_complicated_wires(),
            _ => {
                println!("Invalid entry");
                println!();
            }
        }

        input = String::new();
    }
}

fn process_button() {
    let mut color = get_color();
    while let Err(_) = color {
        println!("Incorrect Input. Try again.");
        color = get_color();
    }
    let color = color.unwrap();

    let mut text = get_text();
    while let Err(_) = text {
        println!("Incorrect Input. Try again.");
        text = get_text();
    }
    let text = text.unwrap();

    if color == "blue" && text == "abort" {
        press_and_hold();
    } else if text == "detonate" && get_batteries() > 1 {
        println!("Press and release the button.");
    } else if color == "white" && label_is_lit("CAR") {
        press_and_hold();
    } else if get_batteries() > 2 && label_is_lit("FRK") {
        println!("Press and release.");
    } else if color == "yellow" {
        press_and_hold();
    } else if color == "red" && text == "hold" {
        println!("Press and release.");
    } else {
        press_and_hold();
    }
}

fn get_color() -> Result<String, String> {
    let mut input = String::new();

    println!("What color is the button?");
    println!("1: Black");
    println!("2: Blue");
    println!("3: Red");
    println!("4: White");
    println!("5: Yellow");
    println!();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => return Err("Failure when retrieving input".to_owned()),
    }
    println!();

    match input.as_str().trim() {
        "1" => Ok("black".to_owned()),
        "2" => Ok("blue".to_owned()),
        "3" => Ok("red".to_owned()),
        "4" => Ok("white".to_owned()),
        "5" => Ok("yellow".to_owned()),
        _ => Err("Invalid value".to_owned()),
    }
}

fn get_text() -> Result<String, String> {
    let mut input = String::new();

    println!("What is the text on the button?");
    println!("1: Abort");
    println!("2: Detonate");
    println!("3: Hold");
    println!("4: Press");
    println!();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => return Err("Invalid entry".to_owned()),
    }
    println!();

    match input.as_str().trim() {
        "1" => Ok("abort".to_owned()),
        "2" => Ok("detonate".to_owned()),
        "3" => Ok("hold".to_owned()),
        "4" => Ok("press".to_owned()),
        _ => Err("Invalid entry".to_owned()),
    }
}

fn get_batteries() -> usize {
    let mut input = String::new();

    println!("How many batteries are there?");
    println!();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => return 0,
    }
    println!();

    match input.trim().parse::<usize>() {
        Ok(num) => num,
        _ => 0,
    }
}

fn label_is_lit(label: &str) -> bool {
    let mut input = String::new();

    println!("Is there a lit indicate with the label \"{}\"?", label);
    println!("y: Yes");
    println!("n: No");
    println!();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => return false,
    }
    println!();

    match input.trim().to_lowercase().as_str() {
        "y" => true,
        _ => false,
    }
}

fn press_and_hold() {
    println!("Press and hold the button. What color is the strip?");
    println!("Blue:\tRelease on 4");
    println!("Yellow:\tRelease on 5");
    println!("Other:\tRelease on 1");
    println!();
}

fn process_password() {
    let mut words = vec!["about", "after", "again", "below", "could",
                        "every", "first", "found", "great", "house",
                        "large", "learn", "never", "other", "place",
                        "plant", "point", "right", "small", "sound",
                        "spell", "still", "study", "their", "there",
                        "these", "thing", "think", "three", "water",
                        "where", "which", "world", "would", "write"];

    let mut column = 1;
    let mut possible_words = Vec::new();

    //loop to add columns
    loop {
        println!("Put in the letters for column {}: ", column);

        let mut column_letters = String::new();
        match io::stdin().read_line(&mut column_letters) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }

        println!();
        //loop to remove all possibilities from words vec if they do not match a letter in the column

        for word in words.iter() {
            //add all words that are compatible to the list of possible words
            for ch in column_letters.to_lowercase().chars() {
                if ch == word.chars().nth(column - 1).unwrap() {
                    possible_words.push(*word);
                }
            }
        }

        //reduce the list of words to just the words that are possible.
        words = possible_words.clone();
        possible_words = Vec::new();

        //print the list of possible words
        for word in words.iter() {
            println!("{}", word);
        }

        column += 1;
        println!();

        if words.len() <= 1 {
            break;
        }
    }
}

fn process_complicated_wires() {
    let mut all_results = String::new();
    let mut num_wires = 0;

    loop {
        println!("Enter the info for a wire as one continuous string. Enter 'q' or blank to quit.");
        println!("y/n for the LED, first letters of colors, and y/n for a star.");

        let mut wire = String::new();
        match io::stdin().read_line(&mut wire) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if wire.trim().contains("q") || wire.trim().len() == 0 {
            break;
        }

        let result = match (wire.trim().chars().nth(0).unwrap(),
                            wire.to_lowercase().contains("r"),
                            wire.to_lowercase().contains("b"),
                            wire.trim().chars().last().unwrap()) {
            ('y', true, true, 'y') => { "D" },
            ('y', true, true, 'n') => { "S" },
            ('y', true, false, 'y') => { "B" },
            ('y', true, false, 'n') => { "B" },
            ('y', false, true, 'y') => { "P" },
            ('y', false, true, 'n') => { "P" },
            ('y', false, false, 'y') => { "B" },
            ('y', false, false, 'n') => { "D" },
            ('n', true, true, 'y') => { "P" },
            ('n', true, true, 'n') => { "S" },
            ('n', true, false, 'y') => { "C" },
            ('n', true, false, 'n') => { "S" },
            ('n', false, true, 'y') => { "D" },
            ('n', false, true, 'n') => { "S" },
            ('n', false, false, 'y') => { "C" },
            ('n', false, false, 'n') => { "C" },
            _ => "?",
        };

        all_results.push_str(result);

        num_wires += 1;
        if num_wires >= 6 {
            break;
        }
    }

    //see if we need to get any additional information
    let mut answer = String::new();
    if all_results.contains("S") {
        //last digit of serial number even?
        println!("Is the last digit of the serial number even? (1 for yes, 2 for no): ");

        match io::stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if answer.trim() == "1" {
            all_results = all_results.replace("S", "C");
        } else if answer.trim() == "2" {
            all_results = all_results.replace("S", "D");
        } else {
            all_results = all_results.replace("S", "?");
        }
    }

    if all_results.contains("P") {
        //does the bomb have a parallel port?
        println!("Is there a parallel port? (1 for yes, 2 for no): ");

        match io::stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if answer.trim() == "1" {
            all_results = all_results.replace("P", "C");
        } else if answer.trim() == "2" {
            all_results = all_results.replace("P", "D");
        } else {
            all_results = all_results.replace("P", "?");
        }
    }

    if all_results.contains("B") {
        println!("Does the bomb have two or more batteries? (1 for yes, 2 for no): ");

        match io::stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if answer.trim() == "1" {
            all_results = all_results.replace("B", "C");
        } else if answer.trim() == "2" {
            all_results = all_results.replace("B", "D");
        } else {
            all_results = all_results.replace("B", "?");
        }
    }

    println!("Results: {}", all_results);
    println!();
}