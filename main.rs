
use std::io;
use rand::Rng;


//I wanted a glorified dice-roll to decide among the things I consider doing
//when I am procrastinating - the problem is a literal dice-roll is inflexible.
//So I made this silly little CLI "tool".
//
//
//only dependency is rand = "0.8.5"

//Compulsive_assembler


fn main() {
    
    clear_terminal();

    let mut input_vector: Vec<String> = Vec::new();
        
    get_items(&mut input_vector);

    pick_item(&input_vector);
    
}


//prompt user input and push as string into Vec<String>
fn get_items(input_vec: &mut Vec<String>)
{

    println!("Enter an item, or enter 'quit' to finish item entry\n");        

    loop{


        let mut user_entry = String::new();    
        
        io::stdin().read_line(&mut user_entry).expect("Line-reading failed");

        user_entry.pop(); //removes trailing '\n'
        
        if user_entry.trim().to_lowercase().eq("quit")
        {
            break;
        }
                
        input_vec.push(user_entry);        
    }

}

//use Rng bounded by size of vector to pick an item supplied by user, 
//out of vector
fn pick_item(item_list: &[String])
{

    //TODO: add exception handling to .unwrap() => None 
    //ie if user enters NO items
    let random_number = rand::thread_rng().gen_range(0..=item_list.len()-1);  

    let choice = item_list.get(random_number).unwrap();

    clear_terminal();

    println!("\nThe algorithm chooses: {}",choice);


}

fn clear_terminal()
{
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
}