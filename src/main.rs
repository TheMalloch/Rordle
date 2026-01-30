use std::{fs::File, io::Read};
use rand::Rng;

const FILEPATH: &str = "src/data/words.txt";
const WORDLEN: u8 = 5;

// This function is returning one word at time as the index ++
fn get_all_words(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
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

fn string_to_vec(string_word: String)-> Result<Vec<char>, Box<dyn std::error::Error>>{ 
    // convert a string to a vector while separating each letter 

    let mut vec = Vec::new();
    for letter in string_word.chars(){
        vec.push(letter);
    }

    return Ok(vec);

}

// This function read a file from a path and load into a dictionnary ?
fn random_word(file_path: &str)-> Result<String, Box<dyn std::error::Error>> {
    // initialize the array (or maybe not)
    //let wordelist: [String;file_lenght(FilePath)] = [_];
    let wordslist= get_all_words(file_path)?; // get the result (vector) containing all the words in the file.
   
    let length_max = wordslist.len();
   
    let num: usize = rand::thread_rng().gen_range(0..=length_max); // return a number between 0 to the maximu lenght of the file.

    let the_choosen_one: String = wordslist[num].clone();   // all case the words is 5 

    return Ok(the_choosen_one); //return the random word in to a vector containing each letter
}

enum PossibleOutcome{
    False, 
    Perfect,
    Almost
}

fn word_check(choosen_word: String, user_word: String)-> Result<(), Box<dyn std::error::Error>>{ // will check every letter of the word
    //create a new vector to house all the letter
    let choosen_vector = string_to_vec(choosen_word)?;
    let user_vector = string_to_vec(user_word)?;

    // compter le nomber de lettre que ce repéte pour le mots choisis et celui de l'utilisateur.
    
    match{
        

    };

    Ok(())
}


fn main(){

    word_check(choosen_word, user_word);

}