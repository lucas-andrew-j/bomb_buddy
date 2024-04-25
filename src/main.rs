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
    let mut wires = 0;

    loop {
        println!("Put in the status of an LED (1 for on, 2 for off): ");

        let mut led = String::new();
        match io::stdin().read_line(&mut led) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if led.trim().contains("q") {
            break;
        }

        println!("Put in the color(s) of the wires (r for red, b for blue): ");

        let mut colors = String::new();
        match io::stdin().read_line(&mut colors) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        println!("Does the wire have a star? (1 for yes, 2 for no): ");

        let mut star = String::new();
        match io::stdin().read_line(&mut star) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        let result = match (led.trim(),
                                colors.to_lowercase().contains("r"),
                                colors.to_lowercase().contains("b"),
                                star.trim()) {
            ("1", true, true, "1") => { "D" },
            ("1", true, true, "2") => { "S" },
            ("1", true, false, "1") => { "B" },
            ("1", true, false, "2") => { "B" },
            ("1", false, true, "1") => { "P" },
            ("1", false, true, "2") => { "P" },
            ("1", false, false, "1") => { "B" },
            ("1", false, false, "2") => { "D" },
            ("2", true, true, "1") => { "P" },
            ("2", true, true, "2") => { "S" },
            ("2", true, false, "1") => { "C" },
            ("2", true, false, "2") => { "S" },
            ("2", false, true, "1") => { "D" },
            ("2", false, true, "2") => { "S" },
            ("2", false, false, "1") => { "C" },
            ("2", false, false, "2") => { "C" },
            _ => "?",
        };

        all_results.push_str(result);

        wires += 1;
        if wires >= 6 {
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
        //does the bomb have two or more batteries?
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

    println!("{}", all_results);
    println!();
}
