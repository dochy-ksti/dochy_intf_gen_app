mod write_file;

use std::path::PathBuf;
use structopt::StructOpt;
use dochy_intf::generate_interface;
use dochy_core::json_dir_to_rust;

#[derive(Debug, StructOpt)]
#[structopt(name = "dochy_intf_gen_app", about = "dochy Interface Generator for Rust. Generates Rust's source code to access dochy Data Management System.")]
struct Opt {
    /// Input directory which contains JSON5 files
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opt : Opt = Opt::from_args();
    let source = match generate_intf_src(opt.input.to_str().unwrap()){
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    match opt.output{
        None =>{ println!("{}", source); }
        Some(p) =>{
            match write_file::write_file(&source, &p){
                Ok(s) => println!("{}", s),
                Err(e) =>{ println!("{}", e.to_string())}
            }
        },
    }
}

fn generate_intf_src(json_dir_path : &str) -> Result<String, String> {
    let mut root =  json_dir_to_rust(json_dir_path, true).or_else(|e| Err(e.to_string()))?;
    Ok(generate_interface(&mut root).to_string())
}


