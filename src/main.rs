use std::io;



///A simple project that works out timings for a meal
///Give instructions and how long they will take, ...
/// ...and the program works out the order and how long to set timers to sync things up correctly
fn main() {

let mut instructions = extract_instructions(get_instruction_list());

instructions.sort_by(|a, b| a.1.cmp(&b.1));
println!("{}", build_instructions(instructions));

}


fn input_in() -> String{
    let mut input : String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");

    return input;
}

fn input_is_valid(input : &String) -> bool{

    for character in input.chars(){
        if character == ':' {
            return true;
        }
    }
    return false;

}

   

fn get_instruction_list() -> Vec<String>{

    let mut instructions = Vec::new();
    println!("Input your instructions followed by the number of minutes they will take as so: [instruction] : [num of minutes]");
    println!("Type 'exit' to continue");
    loop {  
        
        let input = input_in();

        if input.trim().to_lowercase() == "exit".trim().to_string(){
            break;
        } else {
            if input_is_valid(&input) {
                instructions.push(input);
            } else {
                println!("Make sure to input your information correctly in the form [instruction] : [num of minutes]")  
            }

        }
    }

    return instructions;

}

fn extract_instructions(instructions : Vec<String>) -> Vec<(String, i32)> {
    
    let mut instruction_store  = Vec::new();

    for instruction in instructions{
        instruction_store.push(extract_instruction_from_string(instruction));
    }

    return instruction_store;
}


fn extract_instruction_from_string(instruction : String) -> (String, i32){
    

    let mut instruction_part = String::new();
    let mut time_part = String::new();

    let mut record_instruction = true;

    for character in instruction.chars(){
        if character == ':'{
            record_instruction = false;
            continue;
        }


        if record_instruction{
            instruction_part.push(character);
        } else {
            time_part.push(character);
        }
    }

    instruction_part = instruction_part.trim().to_string();


    let time_part = match time_part.trim().parse() {
        Ok(time_part) => time_part,
        Err(_) => 0,
    };


    return (instruction_part, time_part);


}



fn build_instructions(mut instructions : Vec<(String, i32)>) -> String{
    let mut step = 1;
    let mut string_instructions = String::new();

    instructions.reverse();

    for i in 0..instructions.len(){
                        
            let current = instructions[i].to_owned();

            if i > 0 {
                let previous =  instructions[i - 1].to_owned();
                string_instructions.push_str(&work_out_timer(&previous, &current));
                
            }
            string_instructions.push_str(&build_start(&current, step));
            
            if i == instructions.len()-1 {
                string_instructions.push_str(&("⏳ Your final timer to set is ".to_owned() + &current.1.to_string() + " mins\n"));
            }
            
           
            step+=1;
            
        }



    instructions.reverse();
    
    for instruction in &instructions{
    
        string_instructions.push_str(&build_finish(&instruction, step));
        step+=1;
        
    } 
    

    string_instructions.push_str(&("Step ".to_owned() + &step.to_string() + ": Bon appetit!"));



    

    return string_instructions;



} 


fn build_start(instruction : &(String, i32), step : i32) -> String{
    let info = &instruction.0;
    let time = &instruction.1;
    let time_abv_zero = time > &0;
    let mut time_string = String::new();
    if time_abv_zero {
       time_string.push_str(&(time.to_string().to_owned() + " mins"));
    } else {
        time_string.push_str("some amount of time not specified correctly");
    }


    
    return "Step ".to_owned() + &step.to_string() + ": Start \'" + &info.to_string() + "\' which will take " + &time_string + "\n";
}

fn build_finish(instruction : &(String, i32), step : i32) -> String{
    let info = &instruction.0;
    return "Step ".to_owned() + &step.to_string() + ": Finish \'" + info + "\'\n";
}

fn work_out_timer(instruction_one : &(String, i32), instruction_two : &(String, i32)) -> String{
    if instruction_one.1 - instruction_two.1 == 0{
        return "⏳ Don't set a timer! Start on the next instruction\n".to_owned();
    }
    return "⏳ Set a timer for ".to_owned() + &(instruction_one.1 - instruction_two.1).to_string() + " mins\n";
}