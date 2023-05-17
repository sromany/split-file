use clap::{Parser, ValueEnum};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::io::{BufWriter, Write};
use std::ops::RangeBounds;
use std::path::Path;
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

    #[arg(value_enum, short, long, help = "Mode view to select")]
    ///
    pub mode: Mode,
}

impl Cli {
    pub fn fake_test(&self) {
        let current_dir = env::current_dir().unwrap();
        let filepath = current_dir.join(&self.file);

        let mut file = File::open(&filepath).unwrap();
        let seek_end = &file.seek(SeekFrom::End(0)).unwrap();
        file.seek(SeekFrom::Start(0));  

        println!(
            "file: {:?}, SeekEnd: {:?}, Number of line: {:?}",
            &filepath,
            seek_end,
            BufReader::new(&file).lines().count()
        );
    }

    pub fn split_file(&self) -> Result<(), Box<dyn Error>> {
        // TODO: GET lines iteretor
        let mut lines: std::io::Lines<BufReader<File>> =
            self.read_lines_of_file(&self.file).unwrap();

        // TODO: Compute how many lines is the file
        // let num_total_of_line = lines_to_write.len();
        // let num_of_file = (num_total_of_line / MAX_NUM_OF_LINE) + {
        //     (num_total_of_line % MAX_NUM_OF_LINE != 0)
        //         .then_some(1)
        //         .unwrap()
        // };

        // // TODO: SLICE iterator into required lines

        // // Select n row of file and then make a file with new-file-name as base pattern
        // // incremented by one

        // let lines_to_write = lines
        //     .by_ref()
        //     .map(|x| x.expect("Error while unwrapping lines from input file") + "\n")
        //     .collect::<Vec<String>>();

        // println!(
        //     "number of line: {}, Size: {}MB",
        //     num_total_of_line,
        //     file.metadata().unwrap().len() / 1_000_000
        // );
        // println!("Number of split files: {}", num_of_file);

        // let (mut k, mut l) = (0i32, 300_000i32);
        // let mut rev_acc = num_total_of_line as i32 - 300_000i32;
        // for incr in 1..=num_of_file {
        //     let filname = self.file.split(".").into_iter().nth(0).unwrap().to_string();
        //     let extention = self.file.split(".").into_iter().nth(1).unwrap().to_string();
        //     let output_filename = format!(
        //         "{}_{}_{}.{}",
        //         filname,
        //         self.output_pattern.clone().unwrap().to_string(),
        //         incr.to_string(),
        //         extention
        //     );
        //     let output_filepath = current_dir.join(output_filename);
        //     println!("{:?}", output_filepath);
        //     dbg!(incr, k, l, rev_acc);
        //     let batch: Vec<&[u8]> = lines_to_write
        //         .get((k as usize)..(l as usize))
        //         .unwrap()
        //         .iter()
        //         .map(|x| x.as_bytes())
        //         .collect();
        //     let batch_alloc: Vec<u8> = batch.iter().flat_map(|&x| x).copied().collect();
        //     let batch: &[u8] = &batch_alloc;
        //     let mut writer = BufWriter::new(File::create(output_filepath)?);
        //     writer.write_all(batch)?;

        //     k = l;
        //     if rev_acc - MAX_NUM_OF_LINE as i32 > 0i32 {
        //         rev_acc -= MAX_NUM_OF_LINE as i32;
        //         l += MAX_NUM_OF_LINE as i32;
        //     } else if (rev_acc - MAX_NUM_OF_LINE as i32) < 0i32 {
        //         l += rev_acc.abs();
        //     }
        // }
        Ok(())
    }

    pub fn read_lines_of_file<P>(
        &self,
        file: P,
    ) -> Result<std::io::Lines<BufReader<File>>, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let current_dir = env::current_dir().unwrap();
        let filepath = current_dir.join(file);

        println!("file: {:?}", &filepath);

        let file = File::open(&filepath)?;
        return Ok(BufReader::new(file).lines());
    }
}
