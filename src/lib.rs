use std::fs::File;
use std::io::Read;
 
pub fn gender_identification(com: String){
    let mut text_boy = File::open("boysname.txt").expect("boys file not open");
    let mut text_girl = File::open("girlsname.txt").expect("girls file not open"); 
    let mut seperate_boy = String::new();
    let mut seperate_girl = String::new();
    text_boy.read_to_string(&mut seperate_boy).expect("boys file not read");
    text_girl.read_to_string(&mut seperate_girl).expect("girls file not read");
    let compare= com;
    let mut is_found_boy = false;
    let mut is_found_girl = false;
    for word in seperate_boy.split("\r\n"){
        if word==compare{
            is_found_boy = true;
        }   
    }
    for word in seperate_girl.split("\r\n"){
        if word==compare{
            is_found_girl = true;
        }   
    }
    if is_found_boy
        {
            println!("{}: Gender is Male ",compare);
        }
    else if is_found_girl{
        println!("{} : Gender is Female ",compare);
    }
    else{
            println!(".......sorry this name is not our Database.......");
        }
    
}
