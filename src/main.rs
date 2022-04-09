use luhn::is_valid;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    /// Sentence
//    #[clap(short, long)] //to use <cmd> -s, --sentence <SENTENCE> syntax
    code: String,
}


fn main() {

    let args = Args::parse();
    let words=args.code.as_str();
    for w in words.split_whitespace(){
        if w.len() != 4 {
            println!("invalid format");
            return;
        }
    }
    if is_valid(words){ println!("valid");}
    else {println!("invalid");}
}