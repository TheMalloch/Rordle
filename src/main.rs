use std::{fs::File, io::{self, Read},net::TcpListener, num::Saturating};
use rand::Rng;
mod websocket;
//const FILEPATH: &str = "src/data/words.txt";
const DICTPATH: &str = "/home/thedusty/wordle/src/data/dictionnary.txt";
const WORDLEN: u8 = 5;

// This function is returning one word at time as the index ++
pub fn get_all_words(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut getfile = File::open(file_path)?;
    
    /*
    let mut file = match getfile  {
        
        Ok(file) => {
            file
        },
        Err(e) => {
            println!("You dumb as fuck {}",e);
            return;
        },
    };
    */
    let mut content=  String::new();
    
    getfile.read_to_string(&mut content)?;

    let mut filevec = Vec::new();
    for word in content.split_whitespace(){
        filevec.push(word.to_string());
    }

    return Ok(filevec); 
}

pub fn string_to_vec(string_word: String)-> Result<Vec<char>, Box<dyn std::error::Error>>{ 
    // convert a string to a vector while separating each letter 

    let mut vec = Vec::new();
    for letter in string_word.chars(){
        vec.push(letter);
    }

    return Ok(vec);

}
#[allow(warnings)]
fn string_to_vec2(s: String) -> Result<Vec<char>, Box<dyn std::error::Error>> { //claude le batard 

    Ok(s.chars().collect()) // faire une fonction juste pour une boucle "for" faut aller ce faire foutre (ps: pour les dev)
}


// This function read a file from a path and load into a dictionnary ?
pub fn random_word(file_path: &str)-> Result<String, Box<dyn std::error::Error>> {
    // initialize the array (or maybe not)
    //let wordelist: [String;file_lenght(FilePath)] = [_];
    let wordslist= get_all_words(file_path)?; // get the result (vector) containing all the words in the file.
   
    let length_max = wordslist.len();
   
    let num: usize = rand::thread_rng().gen_range(0..=length_max); // return a number between 0 to the maximu lenght of the file.

    let the_choosen_one: String = wordslist[num].clone();   // all case the words is 5 

    return Ok(the_choosen_one); //return the random word in to a vector containing each letter
}

#[derive(Debug)] // besoin de comprendre ce truc
pub enum PossibleOutcome{
    Green,
    Grey,
    Yellow,
}

pub fn word_check(choosen_word: String, user_word: String)-> Result<Vec<PossibleOutcome>, Box<dyn std::error::Error>>{ // will check every letter of the word
    //create a new vector to house all the letter
    let choosen_vector = string_to_vec(choosen_word)?; // convert string to a vector of letters
    let  user_vector= string_to_vec(user_word)?;  // convert string to a vector of letters

    // compter le nomber de lettre que ce repéte pour le mots choisis et celui de l'utilisateur.
    if choosen_vector.is_empty() || user_vector.is_empty(){
        return Err(Box::from("One or both vector are empty!"));
    }

    let mut completion_vector = Vec::new();
    
    for (index, user_letter) in user_vector.iter().enumerate(){
        if user_letter == &choosen_vector[index]{
            // Green: correct letter in correct position
            completion_vector.push(PossibleOutcome::Green);
        } else if choosen_vector.contains(user_letter){
            // Yellow: letter exists but wrong position
            completion_vector.push(PossibleOutcome::Yellow);
        } else {
            // Grey: letter not in word
            completion_vector.push(PossibleOutcome::Grey);
        }
    }

    return Ok(completion_vector);
}

fn record_user_input()-> Result<String, Box<dyn std::error::Error>>{ //record user input for rounds.
    let mut recoded_input = String::new();

    io::stdin().read_line(&mut recoded_input)?;

    let recorded_input = recoded_input.trim().to_string();
    
    return Ok(recorded_input); 
}

// Creation of the dictonnary of valid word. 
// This function will checked if the word does exist in the dictionnary(if in make one). 
// The task is to make a fast parse while checking the words have the right length.

fn load_file(file_path: &str)-> Result<Vec<String>, Box<dyn std::error::Error>>{

    let mut getfile = File::open(file_path)?;
    let mut content = String::new();
    getfile.read_to_string(&mut content)?;
    let mut loaded_vec = Vec::new();

    for w in content.split_whitespace(){
        loaded_vec.push(w.to_string());
    }

    return Ok(loaded_vec);
}

fn does_word_exist(user_input: &String,dictionnary: &Vec<String>)-> bool{
    //Take the user input and checked if it is contained in the file.
    //The file is load once at lauch of the round.
    dictionnary.contains(&user_input)
    //I want to load the file and not having to reload to check every user input.
}


fn rounds(choosen_word: String)-> Result<bool, Box<dyn std::error::Error>>{
    let maxround = 5;
    let mut user_entries = 0;
    let dictionnary = load_file(DICTPATH)?;

    while user_entries < maxround{
        println!("Round {}/{}", user_entries + 1, maxround);
        let user_entry = record_user_input()?;
        
        if user_entry.is_empty() || !does_word_exist(&user_entry, &dictionnary){
            println!("Please enter a valid word!");
            continue;
        }
        
        if user_entry.len() != WORDLEN as usize {
            println!("Word must be {} letters long!", WORDLEN);
            continue;
        }
        
        let results = word_check(choosen_word.clone(), user_entry.clone())?;
        println!("Results: {:?}", results);
        
        // Check if all letters are green (user won)
        if results.iter().all(|outcome| matches!(outcome, PossibleOutcome::Green)){
            println!("You won!");
            return Ok(true);
        }
        
        user_entries += 1;
    }
    
    println!("cheh you twat the word was: {}", choosen_word);
    return Ok(false);
}


fn main() {
}