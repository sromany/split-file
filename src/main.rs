use clap::Parser;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io::{BufWriter, Error, Write};

const MAX_NUM_OF_LINE: i32 = 300_000i32;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long, help = "Path to file to be split")]
    file: String,

    #[arg(short, long, default_value = "split")]
    /// Name of the new files. This will be appended with an incremented number (default: split)
    new_file_name: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap();
    let filepath = current_dir.join(&cli.file);
    println!("file: {:?}", &filepath);
    let file = File::open(&filepath)?;
    let mut reader = BufReader::new(&file);
    let mut lines: std::io::Lines<&mut BufReader<&File>> = reader.by_ref().lines();
    let lines_to_write = lines.by_ref()
        .map(|x| x.expect("Error while unwrapping lines from input file")+"\n")
        .collect::<Vec<String>>();
    dbg!(&lines_to_write.first());
    let num_total_of_line = lines_to_write.len();
    let num_of_file =
        (num_total_of_line / 300_000) + { (num_total_of_line % 300_000 != 0).then_some(1).unwrap() };

    println!("number of line: {}, Size: {}MB", num_total_of_line, file.metadata().unwrap().len() / 1_000_000);
    println!("Number of split files: {}", num_of_file);

    // Select n row of file and then make a file with new-file-name as base pattern
    // incremented by one

    let (mut k, mut l) = (0i32, 300_000i32);
    let mut rev_acc = num_total_of_line as i32 - 300_000i32;
    for incr in 1..=num_of_file {
        let filname = cli.file.split(".").into_iter().nth(0).unwrap().to_string();
        let extention = cli.file.split(".").into_iter().nth(1).unwrap().to_string();
        let output_filename = format!(
            "{}_{}_{}.{}",
            filname,
            cli.new_file_name.clone().unwrap().to_string(),
            incr.to_string(),
            extention
        );
        let output_filepath = current_dir.join(output_filename);
        println!("{:?}", output_filepath);
        dbg!(incr, k,l, rev_acc);
        let bach:Vec<&[u8]> = lines_to_write.get((k as usize)..(l as usize)).unwrap().iter().map(|x| x.as_bytes()).collect();
        let bach_alloc:Vec<u8> = bach.iter().flat_map(|&x| x).copied().collect();
        let batch: &[u8] = &bach_alloc;
        let mut writer = BufWriter::new(File::create(output_filepath)?);
        writer.write_all(batch)?;

        k = l;
        if rev_acc - MAX_NUM_OF_LINE > 0 {
            rev_acc -= MAX_NUM_OF_LINE;
            l += MAX_NUM_OF_LINE;
        } else if rev_acc - MAX_NUM_OF_LINE < 0 {
            l += rev_acc.abs();

        }
    }

    Ok(())
}

