use std::io::{stdin, stdout, Write};

pub fn show_menu(options: Vec<&str>) -> usize {
    let mut width: usize = 0;
    const MARGIN: usize = 4;
    for option in options.iter() {
        width = width.max(option.len());
    }
    width += 3;
    loop {
        println!("{}", "*".repeat(width + MARGIN * 2 + 2));

        for (index, option) in options.iter().enumerate() {
            show_option(index, *option, width, MARGIN);

        }
        println!("{}", "*".repeat(width + MARGIN * 2 + 2));
    
        let mut s: String = String::new();
        print!("Please enter your choice: ");

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        if let Ok(res) = s.trim().parse::<usize>() {
            if res > 0 && res <= options.len() {
                return res;
            }
        }
        println!("Invalid choice, please try again.");
    }
    
}


pub fn show_info(tittle: &str, content: &str){
    let mut width: usize = tittle.len();
    const MARGIN: usize = 4;
    for line in content.lines() {
        width = width.max(line.len());
    }
    width += 1;

    println!("{}", "*".repeat(width + MARGIN * 2 + 2));

    show_line(tittle.to_ascii_uppercase().as_str(), width, MARGIN);

    println!("{}", "*".repeat(width + MARGIN * 2 + 2));

    for line in content.lines() {
        show_line(line, width, MARGIN);

    }

    println!("{}", "*".repeat(width + MARGIN * 2 + 2));

    print!("Press enter to continue");
    let mut s = String::new();
    let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();
}

pub fn ask_for_input(message: &str) -> String{
    print!("{}", message);
    let mut s: String = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s
}


pub fn clear_console(){
    print!("\x1B[2J");
}


fn show_option(index: usize, option: &str, width: usize, margin: usize) {
    let option_with_index: String = format!("{}. {}", index + 1, option);
    let extra_margin: usize = (width.saturating_sub(option_with_index.len())) / 2;

    let separation: usize = margin + extra_margin;
    let alignment: usize = option_with_index.len()%2;
    println!("*{}{}{}*", 
            " ".repeat(separation), 
            option_with_index, 
            " ".repeat(separation + alignment));
}


fn show_line(line: &str, width: usize, margin: usize)
{
    let extra_margin: usize = (width.saturating_sub(line.len())) / 2;

    let separation: usize = margin + extra_margin;
    let alignment: usize = line.len()%2 + 1;
    println!("*{}{}{}*", 
            " ".repeat(separation), 
            line, 
            " ".repeat(separation + alignment));
}