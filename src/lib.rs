use clap::{App, Arg};
use std::error::Error;
use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::fs::File;

type MyResult<T> = Result<T, Box <dyn Error>>;

#[derive(Debug)]
pub struct Config {

    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("uniqr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust uniq")
                    .arg(
                        Arg::with_name("in_file")
                            .value_name("IN_FILE")
                            .help("Input file")
                            .multiple(false)
                            .default_value("-")
                    )
                    .arg(
                        Arg::with_name("out_file")
                            .value_name("OUT_FILE")
                            .help("Output file")
                            .multiple(false)

                    )
                    .arg(

                        Arg::with_name("count")
                            .short("c")
                            .long("count")
                            .help("Show counts")
                            .takes_value(false)
                    )
                    .get_matches();

    let in_file = matches.value_of_lossy("in_file").unwrap().to_string();
    let out_file = matches.value_of_lossy("out_file").map(String::from);
    let count = matches.is_present("count");

    Ok(
        Config{
            in_file,
            out_file,
            count
        }
    )

}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {

    match filename {

        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {


    let mut file = open(&config.in_file)
                    .map_err(|e| format!("{}: {}", config.in_file, e))?;
    
    let mut prev_line = String::new();
    let mut count = 0;
    let mut writer: Box<dyn Write>;
    if config.out_file.is_some() {
        writer = Box::new(BufWriter::new(File::open(config.out_file.unwrap())?));
    }
    else {
        writer = Box::new(BufWriter::new(io::stdout()));
    }

    for actual_line in file.lines() {

        let mut line = actual_line.unwrap();
        
        if line != prev_line && count!=0{

            if !config.count {
                writeln!(writer,"{}", &prev_line);
            }
            else {
                writeln!(writer, "{:>4} {}", count, &prev_line);
            }
            count = 0;
        }
        prev_line = line.clone();
        line.clear();
        count += 1;
    }

    if count == 0 {
        return Ok(());
    }
    if !config.count {
        writeln!(writer,"{}", prev_line);
    }
    else {

        writeln!(writer, "{:>4} {}", count, prev_line);
    }
    writer.flush();
    Ok(())


}