use colored::Colorize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("{}", "No commit message provided".red());
        return;
    }

    let commit_message = &args[1];

    let command = format!("git add --all && git commit -m \"{}\" && git push && git pull", commit_message);
    if args.len() > 3 && &args[3] == "-o" {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .expect("Failed to execute command");
        println!("{}", "Git commands executed successfully".green());
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Failed to execute command");
        println!("{}", "Git commands executed successfully".green());
    }
}
