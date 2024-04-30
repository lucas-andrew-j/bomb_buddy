use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    loop {
        println!("What kind of puzzle?");
        println!("1: Wires");
        println!("2: Button");
        println!("3: Keypad");
        println!("4: Simon Says");
        println!("5: Memory");
        println!("6: Complicated Wires");
        println!("7: Password");
        println!();
        let Ok(_) = io::stdin().read_line(&mut input) else { return };

        println!();

        match input.as_str().trim() {
            "1" => process_wires(),
            "2" => process_button(),
            "3" => process_keypad(),
            "4" => process_simon_says(),
            "5" => process_memory(),
            "6" => process_complicated_wires(),
            "7" => process_password(),
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
        5 => { five_wires(wires) },
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
    if wires.chars().filter(|wires| *wires == 'r').count() > 1 && Ok(false) == serial_is_even() {
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

struct MemoryStage {
    position: u8,
    label: u8,
}

fn process_memory() {
    let mut stage = 1;
    let mut mem = Vec::new();

    loop {
        let mut position: u8 = 0;
        let mut label: u8 = 0;

        println!("Enter the number on the display:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return
        }
        println!();

        match (stage, input.trim()) {
            (1, "3") => position = 3,
            (1, "4") => position = 4,
            (1, _) => position = 2,

            (2, "1") => label = 4,
            (2, "3") => position = 1,
            (2, _) => position = position_from_stage(mem.get(0)),

            (3, "1") => label = label_from_stage(mem.get(1)),
            (3, "2") => label = label_from_stage(mem.get(0)),
            (3, "3") => position = 3,
            (3, "4") => label = 4,

            (4, "1") => position = position_from_stage(mem.get(0)),
            (4, "2") => position = 1,
            (4, _) => position = position_from_stage(mem.get(1)),

            (5, "1") => label = label_from_stage(mem.get(0)),
            (5, "2") => label = label_from_stage(mem.get(1)),
            (5, "3") => label = label_from_stage(mem.get(3)),
            (5, "4") => label = label_from_stage(mem.get(2)),
            _ => return,
        }

        match (position, label) {
            (1..=4, 0) => {
                println!("Select position {}. What is the label?", position);
            },
            (0, 1..=4) => {
                println!("Select label {}. What is the position?", label);
            },
            _ => return,
        }

        if stage == 5 {
            break;
        }

        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return
        }

        let input = match input.trim().parse::<u8>() {
            Ok(x) => x,
            _ => 0,
        };

        println!();

        match (position, label) {
            (1..=4, 0) => label = input,
            (0, 1..=4) => position = input,
            _ => (),
        }

        mem.push(MemoryStage{ position, label });
        stage += 1;
    }

    println!();
}

fn position_from_stage(memory: Option<&MemoryStage>) -> u8 {
    if let Some(stage) = memory {
        stage.position
    } else {
        0
    }
}

fn label_from_stage(memory: Option<&MemoryStage>) -> u8 {
    if let Some(stage) = memory {
        stage.label
    } else {
        0
    }
}

fn process_keypad() {
    let char_map = get_bomb_chars();

    for i in 1..=char_map.len() {
        println!("{:<2} {}", i, char_map.get(&i).unwrap());
    }

    println!();
    println!("Enter the four characters on consecutive lines: ");

    let mut bomb_chars = String::new();

    for _i in 0..4 {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return
        }

        let char_index;
        match input.trim().parse::<usize>() {
            Ok(x) if char_map.contains_key(&(x as usize)) => { char_index = x; },
            _ => return,
        };

        match char_map.get(&char_index) {
            Some(x) => bomb_chars.push(*x),
            _ => return
        }
    }

    println!();

    let mut combinations = get_char_combos();

    for bomb_char in bomb_chars.chars() {
        let mut current_combo = 0;
        for combination in combinations.clone() {
            if !combination.contains(bomb_char) {
                combinations.remove(current_combo);
            } else {
                current_combo += 1;
            }
        }

        if combinations.len() == 1 {
            break;
        }
    }

    let Some(correct_combination) = combinations.get(0) else { return };

    println!("Results: ");
    let mut char_added = 0;
    for i in 0..bomb_chars.chars().count() {
        let mut lowest_char_pos = correct_combination.len();
        let mut lowest_char = ' ';

        for bomb_char in bomb_chars.chars() {
            let position = correct_combination.find(bomb_char).unwrap();

            if (position > char_added || i == 0) && position < lowest_char_pos {
                lowest_char_pos = position;
                lowest_char = bomb_char;
            }
        }

        char_added = lowest_char_pos;
        println!("{} ", lowest_char);
    }

    println!();
}

fn get_bomb_chars() -> HashMap<usize, char> {
    let mut chars = HashMap::new();

    chars.insert(1, 'ټ');
    chars.insert(2, 'Ω');
    chars.insert(3, 'æ');
    chars.insert(4, '©');
    chars.insert(5, 'Ӭ');
    chars.insert(6, 'Ҩ');
    chars.insert(7, 'Ҋ');
    chars.insert(8, 'ϗ');
    chars.insert(9, 'ϰ');
    chars.insert(10, 'Ԇ');
    chars.insert(11, 'Ϙ');
    chars.insert(12, 'Ѯ');
    chars.insert(13, 'ƛ');
    chars.insert(14, 'Ω');
    chars.insert(15, '¶');
    chars.insert(16, 'ψ');
    chars.insert(17, '¿');
    chars.insert(18, 'Ϭ');
    chars.insert(19, 'Ͼ');
    chars.insert(20, 'Ͽ');
    chars.insert(21, '★');
    chars.insert(22, '☆');
    chars.insert(23, '҂');
    chars.insert(24, 'Ѣ');
    chars.insert(25, 'Ѭ');
    chars.insert(26, 'Ѧ');
    chars.insert(27, 'Җ');
    chars.insert(28, 'Ѽ');

    chars
}

fn get_char_combos() -> Vec<String> {
    let mut combinations = Vec::new();
    combinations.push("ϘѦƛϰѬϗϿ".to_owned());
    combinations.push("ӬϘϿҨ☆ϗ¿".to_owned());
    combinations.push("©ѼҨҖԆƛ☆".to_owned());
    combinations.push("Ϭ¶ѢѬҖ¿ټ".to_owned());
    combinations.push("ψټѢϾ¶Ѯ★".to_owned());
    combinations.push("ϬӬ҂æψҊΩ".to_owned());

    combinations
}

fn process_simon_says() {
    let mut answer = String::new();
    
    println!("Does the serial number have a vowel? (y/n)");
    let Ok(_) = io::stdin().read_line(&mut answer) else { return };
    println!();
    let vowel = answer.trim().to_lowercase() == "y";

    answer = String::new();

    println!("How many strikes are there?");
    let Ok(_) = io::stdin().read_line(&mut answer) else { return };
    println!();

    let mut strikes;
    match answer.trim().parse::<usize>() {
        Ok(num) => strikes = num,
        _ => { return },
    }

    let mut new_char = String::new();
    let mut sequence = String::new();
    println!("How color is flashing? (b/r/g/y)");

    loop {
        let Ok(_) = io::stdin().read_line(&mut new_char) else { return };

        match &(new_char.trim().to_lowercase())[..] {
            "m" => strikes += 1,
            "b" | "r" | "g" | "y" => sequence.push_str(&(new_char.trim().to_lowercase())[..]),
            _ => { break; },
        }

        if strikes > 2 {
            break;
        }

        print_sequence(&sequence[..], strikes, vowel);

        println!("Enter the new color (b/r/g/y), 'm' for mistake, or blank for finished:");
        new_char = String::new();
    }

    println!();
}

fn print_sequence(sequence: &str, strikes: usize, vowel: bool) {
    println!();
    let mut mapping: HashMap<char, &str> = HashMap::new();

    match (vowel, strikes) {
        (true, 0) => {
            mapping.insert('r', "Blue");
            mapping.insert('b', "Red");
            mapping.insert('g', "Yellow");
            mapping.insert('y', "Green");
        }, 
        (true, 1) | (false, 2) => {
            mapping.insert('r', "Yellow");
            mapping.insert('b', "Green");
            mapping.insert('g', "Blue");
            mapping.insert('y', "Red");
        }, 
        (true, 2) => {
            mapping.insert('r', "Green");
            mapping.insert('b', "Red");
            mapping.insert('g', "Yellow");
            mapping.insert('y', "Blue");
        }, 
        (false, 0) => {
            mapping.insert('r', "Blue");
            mapping.insert('b', "Yellow");
            mapping.insert('g', "Green");
            mapping.insert('y', "Red");
        }, 
        (false, 1) => {
            mapping.insert('r', "Red");
            mapping.insert('b', "Blue");
            mapping.insert('g', "Yellow");
            mapping.insert('y', "Green");
        },
        _ => { return }
    }

    for ch in sequence.to_lowercase().chars() {
        let Some(color) = mapping.get(&ch) else {return };
        println!("{}", color);
    }

    println!();
}
