use colored::Colorize;


impl Commands {
    fn help(){
        println!("{}","srbxl help".yellow());
        return;
    }
}
struct Commands {
}



fn main() {
    let args: Vec<String> = std::env::args().collect();


    if args.len() == 1 {
        println!("{}","use srbxl help for more information".yellow());
    } // test 1

    if args.len() == 2 {
        if args[1] == "help" {
            Commands::help();

        }
    }

}
