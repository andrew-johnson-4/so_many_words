use std::env;
use so_many_words::util::read_lines;
use so_many_words::lang::Lang;

fn main() {
    let mut text = String::new();
    let args: Vec<String> = env::args().collect();
    for fp in args[1..].iter() {
       if let Ok(lines) = read_lines(fp) {
          for line in lines {
             if let Ok(sent) = line {
                text.push_str(&sent);
                text.push_str("\n");
             }
          }
       }
   }

   let info = whatlang::detect(&text).unwrap();
   let lang: Lang = info.lang().into();
   println!("{}", lang);
}
