use std::io::Write;
use std::io;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::fs::File;
use rand::Rng;
use std::thread;
use std::time::Duration;
use ansi_term::{ANSIString, ANSIStrings};
use ansi_term::Colour::{Red, Green, Yellow, White, Cyan, Blue, Fixed};
use std::collections::HashMap;
use term_size;
fn get_width() -> usize {
    // get the width of terminal, if width cannot be obtained, default to 69 characters
    match term_size::dimensions() {
        Some((w,_h)) => w,
        _ =>  69
    }
}
fn print_error(msg: &str) {
    println!("{}", Red.bold().paint(msg));
    sleep(1200);
}
fn ordered_number(i:i32) -> String {
    match i {
        1 =>  i.to_string() + "st",
        2 => i.to_string() + "nd",
        3 => i.to_string() + "rd",
        4 => i.to_string() + "th",
        5 => i.to_string() + "th",
        _ => String::from("ERROR")
    }
}
// apply the hard mode rules:
// any letter that was in the correct position from last guess must
//      also be correct in guess
// any letter that is present in answer and 
//  last guess must also be present in guess
fn hardmode_validate(answer: &String ,
                     last_guess: &String, 
                     guess: &String) -> String {
    if last_guess == "" {
        return String::new();
    }
    let mut i = 0;
    for ((a,l),g) in answer.chars().zip(last_guess.chars()).zip(guess.chars()) {
        if (a == l) && (l != g) {
            return ordered_number(i+1) + 
                " letter must be " + 
                &a.to_string(); 
        }
        i+=1;
    }
    for a in answer.chars() {
        if last_guess.contains(a) && !guess.contains(a) {
            return String::from("guess must contian ") + &a.to_string();
        }
    }

    String::new() 

}
fn create_keys_map() -> HashMap<char,i8> {
    let mut keys_map: HashMap<char,i8> = HashMap::new();
    for each in "QWERTYUIOPASDFGHJKLZXCVBNM".chars() {
        keys_map.insert(each,0);
    }
    keys_map
}
fn get_next_guess(words: &Vec<String>, 
                  answer: &String,
                  guesses: &Vec<Vec<ANSIString>>,
                  keys_map: &HashMap<char,i8>,
                  last_guess: &String,
                  is_hard_mode: bool) -> String {
    let mut guess: String;
    let mut word_was_added = true;
    let width = get_width();
    loop {
        guess = String::new();
        display_board(guesses, &keys_map, word_was_added, is_hard_mode);
        let prompt = "ENTER GUESS: ";
        print!("{}{}", " ".repeat(width / 2 - (5/2) - prompt.len()),prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).expect(STD_ERR);
        guess = guess.trim().to_uppercase().to_string();
        guess = guess;
        let hardmode_err = hardmode_validate(answer, &last_guess, &guess);
        if guess.len() != 5 {
            print_error(&format!("{}word must be 5 letters long", " ".repeat(width/2 - 20)));
        } else if  !words.contains(&guess) {
            print_error(&format!("{}word is not in my dictionary"," ".repeat(width/2 - 20)));
        } else if is_hard_mode && !hardmode_err.is_empty() {
            print_error(&(" ".repeat(width/2 - hardmode_err.len()/2) + &hardmode_err));
        } else {
            break;
        }
        word_was_added = false;
    }
    guess
}
fn clear() {
    let _ = Command::new("clear").status();
}
fn sleep(milis: u64) {
    thread::sleep(Duration::from_millis(milis));
}
fn get_words() -> Vec<String> {
    let file = match File::open("five_upper") {
        Ok(f) => {
            f
        }
        Err(e) => {
            panic!("{}, {}",e, "cannot find five_upper");
        }
    };
    let reader = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    words
}
fn get_answer(words: &Vec<String>) -> &String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..words.len());
    &words[r]
}
fn display_board(guesses: &Vec<Vec<ANSIString>>,
                 keys_map: &HashMap<char,i8>,
                 has_new_guess: bool,
                 is_hard_mode: bool) {
    clear();
    let width = get_width();
    let header = "WORDLE-CLI".to_owned() + if is_hard_mode {": HARD-MODE"} else {""};
    let padding = " ".repeat((width/2) - header.len()/2);
    println!("{}", Cyan.paint(padding + &header));
    display_keys(&keys_map, width as i32);
    println!();
    if guesses.len() == 0 {
        return;
    }
    if has_new_guess {
        for guess in guesses.iter().take(guesses.len()-1) {
            let word = ANSIStrings(guess);
            println!("{}{}", " ".repeat((width/2) - 5/2), word);
        }
        print!("{}", " ".repeat(width/2 - 5/2));
        for letter in &guesses[guesses.len()-1] {
            sleep(400);
            print!("{}", letter);
            io::stdout().flush().unwrap();
        }
        println!();
    } else {
        for guess in guesses.iter() {
            let word = ANSIStrings(guess);
            println!("{}{}", " ".repeat((width/2) - 5/2), word);
        }
    }
}
fn color_guess(guess : &String, 
               answer : &String, 
               answer_counter : &HashMap<char, i32>,
               keys_map: &mut HashMap<char,i8>) -> Vec<ANSIString<'static>> {
    let mut colored_guess = vec![ANSIString::from("");5];
    let mut matched_indexes = Vec::new();
    let mut counter = answer_counter.clone();
    let bkg = Fixed(235);
    for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate() {
        if !answer.contains(g) {
            keys_map.insert(g,-1);
        }
        if a == g {
             colored_guess[i] = ANSIString::from(Green.on(bkg).paint(a.to_string())); 
             matched_indexes.push(i);
             counter.insert(a,counter.get(&a).unwrap()-1);
             keys_map.insert(a,2);
        }
    }
    for (i,g) in guess.chars().take(5).enumerate() {
        if matched_indexes.contains(&i) {
            continue;
        }     
        if answer.contains(g) 
            && counter.contains_key(&g) 
            && counter.get(&g).unwrap() > &0 {
            colored_guess[i] = ANSIString::from(Yellow.on(bkg).paint(g.to_string())); 
            counter.insert(g, counter.get(&g).unwrap()-1);
            keys_map.insert(g, *keys_map.get(&g).unwrap().max(&1));
        }
        else {
            colored_guess[i] = ANSIString::from(White.on(bkg).paint(g.to_string()));
        }
    }
    colored_guess
}
fn count_answer(answer :&String) -> HashMap<char,i32> {
     let mut counter = HashMap::new();
     for c in answer.chars() {
        if counter.contains_key(&c) {
            counter.insert(c,counter.get(&c).unwrap()+1);
        }
        else {
            counter.insert(c,1);
        }
     }
     counter
}
fn display_keys(keys_map: &HashMap<char,i8>, 
                width: i32) {
    let row1 = vec!['Q','W','E','R','T','Y','U','I','O','P'];
    let row2 = vec!['A','S','D','F','G','H','J','K','L'];
    let row3 = vec!['Z','X','C','V','B','N','M'];
    let keys = vec![row1,row2,row3];
    for row in keys {
        let padding = " ".repeat((width/2) as usize - row.len()/2);
        print!("{}",padding);
        for key in row {
            if *keys_map.get(&key).unwrap() == 0 as i8 {
                print!("{}", White.paint(key.to_string()));
                io::stdout().flush().unwrap();
            }   
            else if *keys_map.get(&key).unwrap() == 1 as i8  {
                print!("{}", Yellow.paint(key.to_string()));
                io::stdout().flush().unwrap();
            }
            else if *keys_map.get(&key).unwrap() == 2 as i8 {
                print!("{}", Green.paint(key.to_string()));
                io::stdout().flush().unwrap();
            } else {
                print!("{}", Red.paint(key.to_string()));
                io::stdout().flush().unwrap();
            }
        }
        println!();
    }
}
const STD_ERR: &str = "error reading from stdin";
fn main() {
    let words = get_words();
    let mut is_hard_mode = false;
    let mut last_guess = String::new();
    let mut keys_map = create_keys_map();
    let mut answer = get_answer(&words);
    let mut answer_counter = count_answer(answer);
    let mut guesses = Vec::new();
    let mut guess_count = 0;

    clear();
    println!("{}",Red.paint("Welcome to Command Line Wordle!"));
    println!("{}", Blue.paint("Press Enter to Play Regulare Mode, or 'H' For Hard Mode"));
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("error reading from stdin");
    if user_input.trim().to_lowercase() == "h" {
        is_hard_mode = true;
    }
    clear();
    loop {
        let mut guess = get_next_guess(&words,
                                       &answer,
                                       &guesses,
                                       &keys_map,
                                       &last_guess,
                                       is_hard_mode);
        guess = guess.to_uppercase();
        let colored_guess = color_guess(&guess, 
                                        &answer, 
                                        &answer_counter,
                                        &mut keys_map);
        guesses.push(colored_guess.clone());

        guess_count+=1;
        if guess == *answer || guess_count == 6 {
            display_board(&guesses, &keys_map, true, is_hard_mode);
            if guess == *answer {
                println!("congrats!  You answered correctly in {} guesses",guess_count);
            }
            if guess_count == 6 {
                println!("The correct word was {}", answer);
            }
            answer = get_answer(&words);
            answer_counter = count_answer(&answer);
            guess_count = 0;
            guesses = Vec::new();
            keys_map = create_keys_map();
            guess = String::new();
            println!("play again? (y/N)");
            let mut y_or_n = String::new();
            io::stdin().read_line(&mut y_or_n).expect(STD_ERR);
            if y_or_n.to_lowercase().trim() != "y" {
                println!("goodbye!");
                break;
            }
        }
        last_guess = guess;
    }
}
