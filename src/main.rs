use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("What kind of puzzle?");
        println!("1: Wires");
        println!("2: Button");
        println!("3: Password");
        println!("4: Complicated Wires");
        println!();
        let Ok(_) = io::stdin().read_line(&mut input) else { return };

        println!();

        match input.as_str().trim() {
            "1" => process_wires(),
            "2" => process_button(),
            "3" => process_password(),
            "4" => process_complicated_wires(),
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
    println!();
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

    loop {
        println!("Put in the letters for column {}: ", column);

        let mut column_letters = String::new();
        match io::stdin().read_line(&mut column_letters) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }

        println!();

        for word in words.iter() {
            for ch in column_letters.to_lowercase().chars() {
                if ch == word.chars().nth(column - 1).unwrap() {
                    possible_words.push(*word);
                }
            }
        }

        words = possible_words.clone();
        possible_words = Vec::new();

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

    println!("Enter the info for a wire as one continuous string. Enter 'q' or blank to quit.");
    println!("y for an LED, first letters of colors, and y for a star.");

    loop {
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
            ('y', true, true, _) => { "S" },
            ('y', true, false, 'y') => { "B" },
            ('y', true, false, _) => { "B" },
            ('y', false, true, 'y') => { "P" },
            ('y', false, true, _) => { "P" },
            ('y', false, false, 'y') => { "B" },
            ('y', false, false, _) => { "D" },
            (_, true, true, 'y') => { "P" },
            (_, true, true, _) => { "S" },
            (_, true, false, 'y') => { "C" },
            (_, true, false, _) => { "S" },
            (_, false, true, 'y') => { "D" },
            (_, false, true, _) => { "S" },
            (_, false, false, 'y') => { "C" },
            (_, false, false, _) => { "C" },
        };

        all_results.push_str(result);

        num_wires += 1;
        if num_wires >= 6 {
            break;
        }

        println!("Next wire ('q' or blank to quit.): ");
    }

    let mut answer = String::new();
    if all_results.contains("S") {
        let answer = serial_is_even();

        if let Ok(true) = answer {
            all_results = all_results.replace("S", "C");
        } else if let Ok(false) = answer {
            all_results = all_results.replace("S", "D");
        } else {
            all_results = all_results.replace("S", "?");
        }
    }

    if all_results.contains("P") {
        println!("Is there a parallel port? (y/n): ");

        match io::stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if answer.trim() == "y" {
            all_results = all_results.replace("P", "C");
        } else if answer.trim() == "n" {
            all_results = all_results.replace("P", "D");
        } else {
            all_results = all_results.replace("P", "?");
        }

        answer.clear();
    }

    if all_results.contains("B") {
        println!("Does the bomb have two or more batteries? (y/n): ");

        match io::stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(_) => return //Err("Failure when retrieving input".to_owned()),
        }
        println!();

        if answer.trim() == "y" {
            all_results = all_results.replace("B", "C");
        } else if answer.trim() == "n" {
            all_results = all_results.replace("B", "D");
        } else {
            all_results = all_results.replace("B", "?");
        }
    }

    println!("Results: {}", all_results);
    println!();
}


fn process_wires() {
    println!("Enter the first letter for each wire in order (but 'u' for blue and 'a' for black)");

    let mut wires = String::new();
    match io::stdin().read_line(&mut wires) {
        Ok(_) => (),
        Err(_) => return
    }
    println!();

    wires = wires.trim().to_lowercase();
    let result = match wires.len() {
        3 => { three_wires(wires) },
        4 => { four_wires(wires) },
        5  => { five_wires(wires) },
        6 => { six_wires(wires) },
        _ => { return },
    };

    if let Ok(x) = result {
        println!("Result: {}", x);
    } else {
        println!("Error when interpreting input.");
    }
    
    println!();
}

fn three_wires(wires: String) -> Result<usize, String> {
    if !wires.contains("r") {
        Ok(2)
    } else if wires.chars().nth(3) == Some('w') {
        Ok(3)
    } else if wires.chars().filter(|wires| *wires == 'u').count() == 2 {
        Ok(wires.rfind('u').unwrap() + 1)
    } else {
        Ok(3)
    }
}

fn four_wires(wires: String) -> Result<usize, String> {
    if wires.chars().filter(|wires| *wires == 'u').count() > 2 && Ok(true) == serial_is_even() {
        Ok(wires.rfind('r').unwrap() + 1)
    } else if wires.chars().nth(3) == Some('y') && !wires.to_lowercase().contains("r") {
        Ok(1)
    } else if wires.chars().filter(|wires| *wires == 'u').count() == 1 {
        Ok(1)
    } else if wires.chars().filter(|wires| *wires == 'u').count() > 1 {
        Ok(4)
    } else {
        Ok(2)
    }
}

fn five_wires(wires: String) -> Result<usize, String> {
    if wires.chars().nth(4) == Some('a') {
        let even_serial = serial_is_even();
        if Ok(false) == even_serial {
            return Ok(4);
        } else if let Err(x) = even_serial {
            return Err(x);
        }
    }
    
    if wires.chars().filter(|wires| *wires == 'r').count() == 1 && wires.chars().filter(|wires| *wires == 'y').count() > 1 {
        Ok(1)
    } else if wires.chars().filter(|wires| *wires == 'a').count() == 0 {
        Ok(2)
    } else {
        Ok(1)
    }
}

fn six_wires(wires: String) -> Result<usize, String> {
    if !wires.to_lowercase().contains("y") {
        let even_serial = serial_is_even();
        if Ok(false) == even_serial {
            return Ok(3);
        } else if let Err(x) = even_serial {
            return Err(x);
        }
    }

    if wires.chars().filter(|wires| *wires == 'y').count() == 1 && wires.chars().filter(|wires| *wires == 'w').count() > 1 {
        Ok(4)
    } else if wires.chars().filter(|wires| *wires == 'r').count() == 0 {
        Ok(6)
    } else {
        Ok(4)
    }
}

fn serial_is_even() -> Result<bool, String> {
    println!("Is the last digit of the serial number even? (y/n): ");
    
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => (),
        Err(_) => return Err("Input failed.".to_owned()),
    }
    println!();

    if answer.trim() == "y" {
        Ok(true)
    } else {
        Ok(false)
    }
}