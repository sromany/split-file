use clap::{Parser, ValueEnum};
use std::io::{BufRead, BufReader, Read};
use std::io::{BufWriter, Write};
use std::{env, error::Error, fs::File};

static MAX_NUM_OF_LINE: usize = 300_000usize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    View,
    Split,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(short, long, help = "Path to file to be split")]
    ///
    file: String,

    #[arg(
        short,
        long,
        default_value = "split",
        help = "Name of the new files. This will be appended with an incremented number (default: split)"
    )]
    ///
    output_pattern: Option<String>,

    //TODO: Options Mutually Exclusive ?
    #[arg(short, long, help = "Option to view")]
    ///
    view_mode: bool,

    #[arg(short, long, help = "Option to view")]
    ///
    split_mode: bool,

    #[arg(value_enum, short, long, help = "Mode view to select", )]
    ///
    pub mode: Mode,
}

impl Cli {

    pub fn is_split_mode(&self) -> bool {
        return self.split_mode;
    }

    pub fn is_view_mode(&self) -> bool {
        return self.view_mode;
    }

    pub fn split_file(&self) -> Result<(), Box<dyn Error>> {
        let current_dir = env::current_dir().unwrap();
        let filepath = current_dir.join(&self.file);
        println!("file: {:?}", &filepath);

        let file = File::open(&filepath)?;
        let mut reader = BufReader::new(&file);
        let mut lines: std::io::Lines<&mut BufReader<&File>> = reader.by_ref().lines();
        let lines_to_write = lines
            .by_ref()
            .map(|x| x.expect("Error while unwrapping lines from input file") + "\n")
            .collect::<Vec<String>>();
        dbg!(&lines_to_write.first());
        let num_total_of_line = lines_to_write.len();
        let num_of_file = (num_total_of_line / MAX_NUM_OF_LINE) + {
            (num_total_of_line % MAX_NUM_OF_LINE != 0).then_some(1).unwrap()
        };

        println!(
            "number of line: {}, Size: {}MB",
            num_total_of_line,
            file.metadata().unwrap().len() / 1_000_000
        );
        println!("Number of split files: {}", num_of_file);

        // Select n row of file and then make a file with new-file-name as base pattern
        // incremented by one

        let (mut k, mut l) = (0i32, 300_000i32);
        let mut rev_acc = num_total_of_line as i32 - 300_000i32;
        for incr in 1..=num_of_file {
            let filname = self.file.split(".").into_iter().nth(0).unwrap().to_string();
            let extention = self.file.split(".").into_iter().nth(1).unwrap().to_string();
            let output_filename = format!(
                "{}_{}_{}.{}",
                filname,
                self.output_pattern.clone().unwrap().to_string(),
                incr.to_string(),
                extention
            );
            let output_filepath = current_dir.join(output_filename);
            println!("{:?}", output_filepath);
            dbg!(incr, k, l, rev_acc);
            let bach: Vec<&[u8]> = lines_to_write
                .get((k as usize)..(l as usize))
                .unwrap()
                .iter()
                .map(|x| x.as_bytes())
                .collect();
            let bach_alloc: Vec<u8> = bach.iter().flat_map(|&x| x).copied().collect();
            let batch: &[u8] = &bach_alloc;
            let mut writer = BufWriter::new(File::create(output_filepath)?);
            writer.write_all(batch)?;

            k = l;
            if rev_acc - MAX_NUM_OF_LINE as i32 > 0i32 {
                rev_acc -= MAX_NUM_OF_LINE as i32;
                l += MAX_NUM_OF_LINE as i32;
            } else if (rev_acc - MAX_NUM_OF_LINE as i32) < 0i32 {
                l += rev_acc.abs();
            }
        }
        Ok(())
    }
}
