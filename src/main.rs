use std::path::Path;
use std::env;
use std::process;

fn main() {
    let input = match JackFile::new() {
        Ok(input) => input,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(2);  
        }
    };


}

struct JackFile {
    path: String // entered by user

}

impl JackFile {
    fn new() -> Result<Self, &'static str> {
        let cmdline_args: Vec<String> = env::args().collect();

        if cmdline_args.len() > 2 {
            return Err("Too many arguments")
        }

        if cmdline_args.len() < 2 {
            return Err("Too less arguments")
        }




        Ok(
            Self { path: cmdline_args[1].clone() }
        )
        
    }

    fn is_directory(&self) -> bool {
        let path = Path::new(&self.path);
        path.is_dir()
    }

    fn get_output_destination(&self) -> String {
        let path = Path::new(&self.path);
        
    }

}
