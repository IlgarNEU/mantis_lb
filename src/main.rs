mod sketch;

use clap::{Parser, Subcommand};
use rand::random;
use sketch::{
    compare_sketches, make_initial_sketch, merge_sketches, read_sketch, read_sketches_from_dir,
    select_most_similar_sketch, sketch_dir_files, write_sketch,
};
use sourmash::signature::SigsTrait;

use crate::sketch::load_ballance_new_fastq_files;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    SketchFiles {
        #[arg(long, default_value_t=String::from("fastq_files"), short='d')]
        fastq_dir: String,
        #[arg(long, default_value_t = 1000, short = 's')]
        scaled: u32,
        #[arg(long, default_value_t = 21, short = 'k')]
        ksize: u32,
    },

    BuildIndex {
        #[arg(long, default_value_t=String::from("fastq_files"), short='d')]
        fastq_dir: String,
        #[arg(long, default_value_t=String::from("initial_index") )]
        sig_dir: String,
        #[arg(long, default_value_t = 5, short = 'n')]
        num_index: u32,
        #[arg(long, default_value_t = 1000, short = 's')]
        scaled: u32,
        #[arg(long, default_value_t = 21, short = 'k')]
        ksize: u32,
    },
    FindMostSimilarIndex {
        fastq_file_path: String,
        #[arg(long, default_value_t=String::from("initial_index"), short='d')]
        sig_dir: String,
        #[arg(long, default_value_t = 1000, short = 's')]
        scaled: u32,
        #[arg(long, default_value_t = 21, short = 'k')]
        ksize: u32,
    },
    LoadBallanceNewFastQ {
        #[arg(long, default_value_t=String::from("fastq_files"), short='d')]
        fastq_dir: String,
        #[arg(long, default_value_t=String::from("initial_index"), short='e')]
        sig_dir: String,
        #[arg(long, default_value_t = 1000, short = 's')]
        scaled: u32,
        #[arg(long, default_value_t = 21, short = 'k')]
        ksize: u32,
        #[arg(long, default_value_t = 5, short = 'n')]
        num_index: u32,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::SketchFiles {
            fastq_dir,
            scaled,
            ksize,
        } => {
            println!("Reading from {a}\nSketching with SourMash!", a = fastq_dir);
            let seed: u64 = random();
            let sketches = sketch_dir_files(&fastq_dir, scaled, ksize, Some(seed));
            let merged = merge_sketches(&sketches, scaled, ksize, seed);
            let filename = "merged.sig";
            println!("Merged sketch contains {} hashes", merged.size());
            write_sketch(filename, &merged);
            let read_merged = read_sketch(filename);
            println!(
                "Read the merged sketch result contains {} hashes",
                read_merged.size()
            );
            let res = compare_sketches(&sketches[0], &sketches[1]);
            println!("similarity {}", res);
        }
        Command::BuildIndex {
            fastq_dir,
            sig_dir,
            num_index,
            scaled,
            ksize,
        } => {
            println!("Building index from {fastq_dir} saving to {sig_dir}");
            make_initial_sketch(&fastq_dir, num_index, scaled, ksize, &sig_dir);
        }
        Command::FindMostSimilarIndex {
            fastq_file_path,
            sig_dir,
            scaled,
            ksize,
        } => {
            let sketches = read_sketches_from_dir(&sig_dir);
            let most_similar_sketch =
                select_most_similar_sketch(&sketches, &fastq_file_path, scaled, ksize);
            println!(
                "Most similar sketch {}, {}",
                most_similar_sketch.0, most_similar_sketch.1
            );
        }
        Command::LoadBallanceNewFastQ {
            fastq_dir,
            sig_dir,
            scaled,
            ksize,
            num_index,
        } => {
            load_ballance_new_fastq_files(&fastq_dir, num_index, scaled, ksize, &sig_dir);
        }
    }
}
