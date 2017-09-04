// function to convert file name to macro for #ifndef #define c++ .h block
// example input: FileManager.h create ifndef block with __FILE_MANAGER_H__
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn to_cpp_ifndef_macro(s :String) -> String{
    let mut s = s;
    s = str::replace(&s, ".", "_");
    let mut start_location = 0;
    let mut words :Vec<String> = Vec::new();
    for (i, c) in s.chars().enumerate() { 
         if c as i32 >= 65 && 90 >= c as i32 && i != 0 {
           let word :String = s.chars().skip(start_location).take(i - start_location).collect();
           words.push(word.to_uppercase());
           start_location = i;
         }    
    }
    let word :String = s.chars().skip(start_location).take(s.len()- start_location).collect();
    words.push(word.to_uppercase());
    
    let mut out_case = String::from("_");
    for word in words {
        out_case.push_str("_");
        out_case.push_str(&word);
    }   
    out_case.push_str("__");
    return out_case;
}

fn main() {
    let args: Vec<_> = env::args().collect();    
    if args.len() != 2 {
        println!("invalid number of arguments.");
        return;
    }
    
    let path = Path::new(&args[1]);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let data = format!("#ifndef {name}\r\n#define {name}\r\n\r\n#endif /*{name}*/\r\n",name=to_cpp_ifndef_macro(file_name));
    
    let metadata = fs::metadata(args[1].clone()).unwrap();
    if metadata.len() != 0 {
        println!("file is not empty.");
        return;
    }
    let mut f = File::create(args[1].clone()).unwrap();
    f.write_all(data.as_bytes()).expect("io error");
}