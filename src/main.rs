use termcolor::{Color, ColorChoice, ColorSpec, Buffer, WriteColor, StandardStream};
use std::io::Write;
use std::io;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::fs::File;
use rand::Rng;
use std::thread;
use std::time::Duration;
use ansi_term::{ANSIString, ANSIStrings};
use ansi_term::Colour::{Red, Green, Yellow, White};
use std::collections::{HashMap};
fn get_next_guess(words: &Vec<String>, 
                  guesses: &Vec<Vec<ANSIString>>
                  ) -> String {
    let mut guess = String::new();
    let mut word_was_added = true;
    loop {
        guess = String::new();
        display_board(guesses, word_was_added);
        io::stdin().read_line(&mut guess).expect("wtf");
        guess = guess.trim().to_uppercase().to_string();
        guess = guess;
        if guess.len() != 5 {
            println!("{}", Red.paint("word must be 5 letters long"));
            sleep(1000);
        } else if  !words.contains(&guess) {
            println!("{}", Red.paint("word is not in my dictionary"));
            sleep(1000);
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
fn display_board(guesses: &Vec<Vec<ANSIString>>, has_new_guess: bool) {
    clear();
    if guesses.len() == 0 {
        return;
    }
    if has_new_guess {
        for guess in guesses.iter().take(guesses.len()-1) {
            let word = ANSIStrings(guess);
            println!("{}", word);
        }
        for letter in &guesses[guesses.len()-1] {
            sleep(400);
            print!("{}", letter);
            io::stdout().flush().unwrap();
        }
        println!();
    } else {
        for guess in guesses.iter() {
            let word = ANSIStrings(guess);
            println!("{}", word);
        }
    }
}
fn color_guess(guess : &String, 
               answer : &String, 
               answer_counter : &HashMap<char, i32>) -> Vec<ANSIString<'static>> {
    let mut colored_guess = vec![ANSIString::from(""),ANSIString::from(""),ANSIString::from(""),ANSIString::from(""),ANSIString::from("")];
    let mut matched_indexes = Vec::new();
    let mut counter = answer_counter.clone();
    for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate() {
        if a == g {
             colored_guess[i] = ANSIString::from(Green.paint(a.to_string())); 
             matched_indexes.push(i);
             counter.insert(a,counter.get(&a).unwrap()-1);
        }
    }
    for (i,g) in guess.chars().take(5).enumerate() {
        if matched_indexes.contains(&i) {
            continue;
        }     
        if answer.contains(g) 
            && counter.contains_key(&g) 
            && counter.get(&g).unwrap() > &0 {
            colored_guess[i] = ANSIString::from(Yellow.paint(g.to_string())); 
            counter.insert(g, counter.get(&g).unwrap()-1);
        }
        else {
            colored_guess[i] = ANSIString::from(White.paint(g.to_string()));
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
fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut red = ColorSpec::new();
    red.set_fg(Some(Color::Red)).set_bold(true);
    let mut blue = ColorSpec::new();
    blue.set_fg(Some(Color::Blue));
    let mut white = ColorSpec::new();
    white.set_fg(Some(Color::White));

    stdout.set_color(&red).unwrap();
    writeln!(&mut stdout, "Welcome to Command Line Wordle!").unwrap();
    stdout.reset().unwrap();
    stdout.set_color(&blue).unwrap();
    writeln!(&mut stdout, "Press any key to continue").unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("wtf");
    clear();
    let words = get_words();
    let mut answer = get_answer(&words);
    let mut answer_counter = count_answer(answer);
    stdout.set_color(&white).expect("wtf");
    writeln!(&mut stdout,"{}", answer).unwrap();
    let mut guess = String::new();
    let mut guesses = Vec::new();
    let mut guess_count = 0;
    loop {
        let mut guess = get_next_guess(&words,&guesses);
        guess = guess.to_uppercase();
        let colored_guess = color_guess(&guess, &answer, &answer_counter);
        guesses.push(colored_guess.clone());

        guess_count+=1;
        if guess == *answer || guess_count == 6 {
            display_board(&guesses, true);
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
            println!("play again? (y/N)");
            let mut y_or_n = String::new();
            io::stdin().read_line(&mut y_or_n).expect("wtf");
            if y_or_n.to_lowercase().trim() != "y" {
                println!("goodbye!");
                break;
            }

        }
    }
}
