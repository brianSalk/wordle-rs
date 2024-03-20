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
use std::env;
use term_size;
struct LanguageStuff {
    language: String,
    welcome: String,
    press_enter: String,
    err_not_five: String,
    err_not_in_dict: String,
    play_again: String,
    guess_prompt: String,
    hardmode_val: fn(&String,&String,&String) -> String,
    get_win_msg: fn(i32) -> String,
    get_loser_msg: fn(answer: &String) -> String,
    create_keys_mp: fn() -> HashMap<char,i8>,
    display_kys: fn(&HashMap<char,i8>, i32),
    keep_plyg: fn(&String) -> bool,
    goodbye: String,
}
/// contains functions and string members that are specific to each supported language (aupported
/// languages are English and German)
impl LanguageStuff {
   fn german() -> Self {
        Self {
            language: String::from("de"),
            welcome: String::from("Willkommen zu Command-Line-Wordle!"),
            press_enter: String::from("die Eingabetaste Drücken für Normal-Mode, oder 'H' eingeben für Schwer-Mode"),
            err_not_five: String::from("Die Vermutung muss fünf Buchstaben enthalten"),
            err_not_in_dict: String::from("Das Wort erkenne ich nicht"),
            play_again: String::from("Wollen Sie wieder spielen? (j/N) "),
            guess_prompt: String::from("VERMUTUNG EINGEBEN: "),
            hardmode_val: hardmode_validate_german,
            get_win_msg: get_win_message_german,
            get_loser_msg: get_loser_message_german,
            create_keys_mp: create_keys_map_german,
            display_kys: display_keys_german,
            keep_plyg: keep_playing_german,
            goodbye: String::from("Tschüß!")
        }
   } 
   fn english() -> Self {
       Self {
           language: String::from("en"),
           welcome: String::from("Welcome to Command Line Wordle!"),
           press_enter: String::from("Press Enter to Play Regulare Mode, or 'H' For Hard Mode"),
           // replace these when finished
            err_not_five: String::from("word must be 5 letters long"),
            err_not_in_dict: String::from("word is not in my dictionary"),
            play_again: String::from("play again? (y/N) "),
            guess_prompt: String::from("ENTER GUESS: "),
            hardmode_val: hardmode_validate_english,
            get_win_msg: get_win_message_english,
            get_loser_msg: get_loser_message_english,
            create_keys_mp: create_keys_map_english,
            display_kys: display_keys_english,
            keep_plyg: keep_playing_english,
            goodbye: String::from("Goodbye!")
       }
   }
   fn hardmode_validate(&self,a: &String, b: &String, c: &String) -> String {
        return (self.hardmode_val)(a,b,c)
   }
   fn get_win_message(&self,count : i32) -> String {
        return (self.get_win_msg)(count);
   }
   fn get_loser_message(&self, answer : &String) -> String {
        return (self.get_loser_msg)(answer);
   }
   fn create_keys_map(&self) -> HashMap<char,i8> {
        return (self.create_keys_mp)();
   }
   fn display_keys(&self, keys: &HashMap<char, i8>, width: i32) {
        return (self.display_kys)(keys, width);
   }
   fn keep_playing(&self, ans: &String) -> bool {
       return (self.keep_plyg)(ans);
   }
}
fn to_uppercase_modified(s: &mut String) -> String {
    let res:String;
    if s.contains("ß") {
        let split_s: Vec<&str> = s.split("ß").collect();
        res = split_s[0].to_uppercase().trim().to_owned() + "ß" + &split_s[1].trim().to_uppercase();
    }
    else {
        res =  s.to_uppercase().to_string();
    }
    res.to_owned()
}
/// return a LanguageStuff struct for a given language based on LANG environment variable.
/// default is english
fn get_language_stuff() -> LanguageStuff {
    let language = env::var("LANG").expect("en");
    if language.starts_with("en") {
        return LanguageStuff::english();
    }
    else if language.starts_with("de") {
        return LanguageStuff::german();
    }
    else {
        return LanguageStuff::english();
    }
}
/// `s` string to be centered
fn to_centered(s: & str) -> String {
    if s.len() < get_width() {
        let padding = " ".repeat(get_width() / 2 - s.len() /2).to_owned();
        return padding + s;
    }
    return s.to_string();
}
/// return true if `ans` is J or j, false otherwise
fn keep_playing_german(ans: &String) -> bool {
    return ans.to_lowercase().trim() == "j";
}
/// return true if `ans` is Y or y, false otherwise
fn keep_playing_english(ans: &String) -> bool {
    return ans.to_lowercase().trim() == "y";
}
/// get width of terminal, default to 69.
fn get_width() -> usize {
    // get the width of terminal, if width cannot be obtained, default to 69 characters
    match term_size::dimensions() {
        Some((w,_h)) => w,
        _ =>  69
    }
}
/// print error message formatted 
fn print_error(msg: &str) {
    // center the message
    let width = get_width();
    let padded_msg = " ".repeat(width/2 - msg.len()/2) + msg;
    println!("{}", Red.bold().paint(padded_msg));
    sleep(1200);
}
/// return string of number in ordinal number form
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
/// return message telling user they won in english
fn get_win_message_english(guess_count: i32) -> String {
    format!("{} {} {}", "congrats!  You answered correctly in", guess_count, "guesses")
}
/// return message telling user they won in german
fn get_win_message_german(guess_count: i32) -> String {
    format!("{} {} {}", "gut gemacht!  Sie haben in", guess_count, "Vermuten gewonnen")
}
/// return loser message in english
fn get_loser_message_english(answer :&String) ->String {
    return format!("{} {}", "The correct word was", answer);
}
/// return loser message in german
fn get_loser_message_german(answer :&String) ->String {
    return format!("{} {}", "Die richtige Antwort war", answer);
}
/// validate guess according to hardmode rules
fn hardmode_validate_english(answer: &String ,
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
/// validate guess according to hard mode rules in german
fn hardmode_validate_german(answer: &String ,
                     last_guess: &String, 
                     guess: &String) -> String {
    if last_guess == "" {
        return String::new();
    }
    let mut i = 0;
    for ((a,l),g) in answer.chars().zip(last_guess.chars()).zip(guess.chars()) {
        if (a == l) && (l != g) {
            return (i+1).to_string() + 
                ". Buchstabe muss " + 
                &a.to_string() + " sein"; 
        }
        i+=1;
    }
    for a in answer.chars() {
        if last_guess.contains(a) && !guess.contains(a) {
            return String::from("Vermutung muss ") + &a.to_string() + " enthalten";
        }
    }
    String::new() 

}
/// create keys to be displayed in terminal
fn create_keys_map_english() -> HashMap<char,i8> {
    let mut keys_map: HashMap<char,i8> = HashMap::new();
    for each in "QWERTYUIOPASDFGHJKLZXCVBNM".chars() {
        keys_map.insert(each,0);
    }
    keys_map
}
/// create keys to be displayed in termanisl, include special german characters
fn create_keys_map_german() -> HashMap<char,i8> {
    let mut keys_map: HashMap<char,i8> = HashMap::new();
    for each in "QWERTZUIOPÜASDFGHJKLÖÄYXCVBNMß".chars() {
        keys_map.insert(each,0);
    }
    keys_map
}
/// prompt and get the next guess from the user
fn get_next_guess(words: &Vec<String>, 
                  answer: &String,
                  guesses: &Vec<Vec<ANSIString>>,
                  keys_map: &HashMap<char,i8>,
                  last_guess: &String,
                  is_hard_mode: bool,
                  language_stuff: &LanguageStuff) -> String {
    let mut guess: String;
    let mut word_was_added = true;
    let width = get_width();
    loop {
        guess = String::new();
        display_board(guesses, &keys_map, word_was_added, is_hard_mode, &language_stuff);
        let prompt = &language_stuff.guess_prompt;
        print!("{}{}", " ".repeat(width / 2 - (5/2) - prompt.len()),prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).expect(STD_ERR);
        guess = to_uppercase_modified(&mut guess).trim().to_owned();
        let hardmode_err = language_stuff.hardmode_validate(&answer, &last_guess, &guess);
        if guess.chars().count() == 0 {
            // DO NOTHING IF GUESS IS EMPTY
        } else if guess.chars().count() != 5 {
            print_error(&language_stuff.err_not_five);
        } else if  !words.contains(&guess) {
            print_error(&language_stuff.err_not_in_dict);
        } else if is_hard_mode && !hardmode_err.is_empty() {
            print_error(&hardmode_err);
        } else {
            break;
        }
        word_was_added = false;
    }
    guess
}
/// clear terminal
fn clear() {
    let _ = Command::new("clear").status();
}
/// sleep for millisecs
fn sleep(milis: u64) {
    thread::sleep(Duration::from_millis(milis));
}
/// read from one of the text files of words.  if english, this removes most
/// plural and third-person conjugated verbs such as 'makes' and 'likes'
fn get_words_for_answer(language: &String) -> Vec<String> { 
    let file_to_open = if language == "en" { "five_upper.txt" } else if language == "de" { "five_upper_german.txt" } else { "en" };
    let file = match File::open(file_to_open) {
        Ok(f) => {
            f
        }
        Err(e) => {
            panic!("{}, {}",e, "cannot find ".to_owned() + file_to_open);
        }
    };
    let reader = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    words
}
/// read from one of the text files of words.  For languages other than English, this
/// should return the same words as get_words_for_answer 
fn get_words_for_guesses(language: &String) -> Vec<String> { 
    let file_to_open = if language == "en" { "five_upper_guesses.txt" } else if language == "de" { "five_upper_german.txt" } else { "en" };
    let file = match File::open(file_to_open) {
        Ok(f) => {
            f
        }
        Err(e) => {
            panic!("{}, {}",e, "cannot find ".to_owned() + file_to_open);
        }
    };
    let reader = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    words
}
/// select a random answer from the list of words
fn get_answer(words: &Vec<String>) -> &String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..words.len());
    &words[r]
}
/// print wordle board to the screen
fn display_board(guesses: &Vec<Vec<ANSIString>>,
                 keys_map: &HashMap<char,i8>,
                 has_new_guess: bool,
                 is_hard_mode: bool,
                 language_stuff: &LanguageStuff) {
    clear();
    let width = get_width();
    let header = "WORDLE-CLI".to_owned() + if is_hard_mode {": HARD-MODE"} else {""};
    let padding = " ".repeat((width/2) - header.len()/2);
    println!("{}", Cyan.paint(padding + &header));
    language_stuff.display_keys(&keys_map, width as i32);
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
/// for each char in guess color it: white if absent, yellow if wrong position and green if correct
/// position
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
/// create and return map of number of occurances of each char in answer
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
/// print keys to screen
fn display_keys_english(keys_map: &HashMap<char,i8>, width: i32) {
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
/// print german keys to screen
fn display_keys_german(keys_map: &HashMap<char,i8>, width: i32) {
    let row1 = vec!['Q','W','E','R','T','Y','U','I','O','P','Ü'];
    let row2 = vec!['A','S','D','F','G','H','J','K','L','Ö', 'Ä'];
    let row3 = vec!['Z','X','C','V','B','N','M', 'ß'];
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
/// error message when stdin error occures (should never happen!!)
const STD_ERR: &str = "error reading from stdin";
fn main() {
    ctrlc::set_handler(move || {
        println!();
        std::process::exit(1);
    });
    let language_stuff:LanguageStuff = get_language_stuff();
    let words_guesses = get_words_for_guesses(&language_stuff.language);
    let words_answer = get_words_for_answer(&language_stuff.language);
	
    let mut is_hard_mode = false;
    let mut last_guess = String::new();
    let mut keys_map = language_stuff.create_keys_map();
    let mut answer = get_answer(&words_answer);
    let mut answer_counter = count_answer(answer);
    let mut guesses = Vec::new();
    let mut guess_count = 0;

    clear();
    println!("{}",Red.paint(to_centered(&language_stuff.welcome)));
    println!("{}", Blue.paint(to_centered(&language_stuff.press_enter)));
    let mut user_input = String::new();
    print!("{}", to_centered(""));
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("error reading from stdin");
    if user_input.trim().to_lowercase() == "h" {
        is_hard_mode = true;
    }
    clear();
    loop {
        let mut guess = get_next_guess(&words_guesses,
                                       &answer,
                                       &guesses,
                                       &keys_map,
                                       &last_guess,
                                       is_hard_mode,
                                       &language_stuff);
        guess = to_uppercase_modified(&mut guess);
        let colored_guess = color_guess(&guess, 
                                        &answer, 
                                        &answer_counter,
                                        &mut keys_map);
        guesses.push(colored_guess.clone());

        guess_count+=1;
        if guess == *answer || guess_count == 6 {
            display_board(&guesses, &keys_map, true, is_hard_mode, &language_stuff);
            if guess == *answer {
                let congrats = language_stuff.get_win_message(guess_count);
                println!("{}",to_centered(&congrats));
            }
            else if guess_count == 6 {
                let loser_message = language_stuff.get_loser_message(answer);
                println!("{}",to_centered(&loser_message));
            }
            answer = get_answer(&words_answer);
            answer_counter = count_answer(&answer);
            guess_count = 0;
            guesses = Vec::new();
            keys_map = language_stuff.create_keys_map();
            guess = String::new();
            print!("{}",to_centered(&language_stuff.play_again));
            io::stdout().flush().unwrap();
            let mut y_or_n = String::new();
            io::stdin().read_line(&mut y_or_n).expect(STD_ERR);
            if !language_stuff.keep_playing(&y_or_n) {
                println!("{}",to_centered(&language_stuff.goodbye));
                break;
            }
        }
        last_guess = guess;
    }
}
