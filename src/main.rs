use lalrpop_util::lalrpop_mod;
use std::env;

lalrpop_mod!(parser);
mod ast;
mod pdf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: lobstex file.lob");
        return;
    }

    let file_path = &args[1];
    let code = std::fs::read_to_string(file_path).unwrap();
    let document = parser::DocumentParser::new().parse(code.as_str()).unwrap();
    //dbg!(&document);

    let file_path = std::path::PathBuf::from(file_path);
    pdf::render_pdf(file_path, document);
}
