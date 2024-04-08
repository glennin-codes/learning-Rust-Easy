
use std::{ fs, i32, io, num};

#[derive(Debug)]
enum CliError{
  IoError(io::Error),
  ParseError(num::ParseIntError)

}
impl From <io::Error> for  CliError{
  fn from(e: io::Error) -> Self {
      CliError::IoError(e)
  }
}
impl From <num::ParseIntError> for CliError{
  fn from(e:num::ParseIntError)->Self{
    CliError::ParseError(e)
  }
}
fn open_and_parse_file(file_name:&str)->Result<i32,CliError>{
  
  let contents=fs::read_to_string(&file_name)?;
  println!("{}",contents);
  let num:i32=contents.trim().parse()?;
 Ok(num)
}

fn main(){
  let value = open_and_parse_file("README.md");

  match value{
    Ok(num) => println!(" value {}",num),
    Err(e) => println!(" error  {:?}",e)
  }
}