use clap::Parser;
use dialoguer::Input;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: Option<String>,
    
    #[arg(short, long)]
    output: Option<String>,
}

fn get_args() -> (String, Option<String>) {
    let Args {
        file_name: w_file_name,
        output: w_output,
    } = Args::parse();

    let file_name = if let Some(file_name) = w_file_name {
        file_name
    } else {
        Input::<String>::new()
            .with_prompt("File name")
            .interact_text()
            .expect("Failed to get file name")
    };

    (file_name, w_output)
}

fn main() {
    let (file_name, w_output) = get_args();

    let contents = fs::read_to_string(&file_name).expect("Something went wrong reading the file");

    let fixed_contents = contents
        .replace("。\n", ".\n")
        .replace("、\n", ",\n")
        .replace("、", ", ")
        .replace("。", ". ")
        .trim_end()
        .to_string();

    let target_file_name: &str = if let Some(output) = w_output.as_ref() {
        output
    } else {
        &file_name
    };

    let mut file = fs::File::create(&target_file_name).expect("Failed to create file");

    file.write_all(fixed_contents.as_bytes())
        .expect("Failed to write to file");
}