pub mod tokenizer;

/*
let mut predefined_name_order = vec![
    ("LET", ""),
    ""
];
*/

pub fn exec(input: String) {
    let token_collection = tokenizer::make_tokens(input);

    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
        match value {
            tokenizer::ValueEnum::String(v) => {
                println!("ERROR: {}", v);
                return;
            },
            tokenizer::ValueEnum::IntegerVector(_v) => (),
            tokenizer::ValueEnum::StringVector(_v) => ()
        }
    }
    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
        match value {
            tokenizer::ValueEnum::String(_v) => {
                println!("");
                return;
            },
            tokenizer::ValueEnum::IntegerVector(_v) => (),
            tokenizer::ValueEnum::StringVector(_v) => ()
        }
    }

    println!("{:?}", token_collection);
}    

pub fn reset() {
    //
}

pub fn stop() {
    //
}
