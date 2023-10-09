use std::fmt::format;

fn get_rem_euclid_m() -> i32{
    return ('z' as u8 - 'a' as u8 + 1) as i32;
}

fn char_to_number(c: char) -> i32{
    let res = (c as u8 - 'a' as u8) as i32;
    return res;
}

fn number_to_char(x: i32) -> char{
    return (x + 'a' as u8 as i32) as u8 as char
}


fn shift_character(c: char, n: i32) -> char{
    let c_as_number = (c as u8 - 'a' as u8) as i32;
    let m = get_rem_euclid_m();
    let shifted_c_as_number = (c_as_number + n).rem_euclid(m);
    return (shifted_c_as_number + 'a' as u8 as i32) as u8 as char;
}


fn multiply_string(key: &String, n: i32) -> String {
    let res = (0..=n).map(|x| "".to_string())
        .reduce(|acc, _i| [acc, key.clone()].join(""));
    return res.expect("this should never happen");
}

fn get_key_with_length(key: &String, n: i32) -> String {
    let multiply_times = n / key.len() as i32;
    let new_key = multiply_string(key, multiply_times + 1).chars()
        .take(n as usize)
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");
    return new_key;
}


fn encrypt_vigenere(key: &String, text: &String) -> String {
    let new_key = get_key_with_length(key, text.len() as i32);
    let res = text.chars().zip(new_key.chars())
        .map(|(a, b)| shift_character(a, char_to_number(b)))
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");

    return res;
}

fn decrypt_vigenere(key: &String, text: &String) -> String {
    let new_key = get_key_with_length(key, text.len() as i32);
    let res = text.chars().zip(new_key.chars())
        .map(|(a, b)| shift_character(a, -1 * char_to_number(b)))
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");

    return res;
}

fn z1(){
    let key = "CIPHER".to_string();
    println!("key: {key}");
    println!("Encryption");

    let plaintext = "thiscryptosystemisnotsecure".to_string();

    println!("Plaintext: {plaintext}");
    let cyphertext = encrypt_vigenere(&key.to_lowercase(), &plaintext);
    println!("Cyphertext: {}", cyphertext.to_uppercase());


    println!("Decryption");
    let decrypted = decrypt_vigenere(&key.to_lowercase(), &cyphertext);
    println!("Plaintext: {decrypted}");
    println!("Cyphertext: {}", cyphertext.to_uppercase());
}

fn main() {
    // println!("Hello, world!");

    z1();

}
