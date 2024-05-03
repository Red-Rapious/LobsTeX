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
    parser::ExprParser::new().parse(code.as_str()).unwrap();
    let file_path = std::path::PathBuf::from(file_path);

    pdf::render_pdf(file_path);
}
