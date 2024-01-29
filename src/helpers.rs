use rand::Rng;


pub fn new_name() -> String {

    let mut rng = rand::thread_rng();
    let vowels: [&str;5] = ["a", "e", "i", "o", "u"];
    let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z"];

    let mut r_name = String::from("");
    let name_length = rng.gen_range(5..10);
    for i in 0..name_length {
        if i % 2 == 0 {
            let vowel = vowels[rng.gen_range(0..vowels.len())];
            r_name+= vowel 
        } else {
            let consonant = consonants[rng.gen_range(0..consonants.len())];
            r_name += consonant;
        }

    }

    r_name
}

