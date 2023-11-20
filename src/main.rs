use std::{fs::File, io::{BufRead, BufReader}};
use clap::{App, Arg};
use jsonwebtoken::{decode, Algorithm, Validation};

fn main() {
    let matches = App::new("jvvt")
        .version("1.0")
        .author("7a6136")
        .about("Crack JWT using dictionary attack")
        .arg(Arg::new("token")
            .short('t')
            .long("token")
            .value_name("TOKEN")
            .about("Sets the JWT token")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .about("Sets the dictionary file")
            .takes_value(true)
            .required(true))
        .get_matches();

    let token = matches.value_of("token").unwrap();
    let dictionary_file = matches.value_of("file").unwrap();

    let file = File::open(dictionary_file).expect("無法打開檔案");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let secret = line.expect("無法讀取行");
        let validation = Validation::new(Algorithm::HS256, secret.as_bytes());

        match decode::<serde_json::Value>(&token, secret.as_bytes(), &validation) {
            Ok(_) => {
                println!("找到正確的密鑰: {}", secret);
                break;
            },
            Err(_) => continue,
        }
    }
}
