use std::io::{BufWriter, Write};
use clap::Parser;
use indicatif::ProgressStyle;
use rand::Rng;
use std::time::{Duration, Instant};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::fs::{self, File};
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;


const NONCE_SIZE: usize = 6;
const HASH_SIZE: usize = 10;
const RECORD_SIZE: usize = NONCE_SIZE + HASH_SIZE;

// Structure to hold a 16-byte record
#[derive(Debug)]
struct Record {
    hash: [u8; HASH_SIZE], // hash value as byte array
    nonce: [u8; NONCE_SIZE], // Nonce value as byte array
}

// Adding implementation to create a new Record
impl Record {
    fn new(hash: [u8; HASH_SIZE], nonce: [u8; NONCE_SIZE]) -> Self {
        Record { hash, nonce }
    }
}

// Cli format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    ///  Specify the filename
    #[arg(short, long)]
    filename: String,
    
    /// Specify Number of Hash Threads
    #[arg(short='t')]
    num_threads_hash: usize,
    
    /// Specify the number of Sort Threads
    #[arg(short='o')]
    num_threads_sort: usize,
    
    /// Specify the number of Write Threads
    #[arg(short='i')]
    num_threads_write: usize,
    
    /// Specify the maximum amount of memory to be used in MB
    #[arg(short, long)]
    memory_size: i64,
    
    /// Specify the filesize in MB
    #[arg(short='s')]
    filesize: i64,
    
    /// Turns on debug mode with true, off with false
    #[arg(short, long)]
    debug: Option<String>,
}

// Print arguments passed through the command line
fn print_args(cli: &Cli, memory_limit_in_bytes: &i64) {
    println!("NUM_THREADS_HASH={}", cli.num_threads_hash);
    println!("NUM_THREADS_SORT={}", cli.num_threads_sort);
    println!("NUM_THREADS_WRITE={}", cli.num_threads_write);
    println!("FILENAME={}", cli.filename);
    println!("MEMORY_SIZE={}MB", cli.memory_size);
    println!("FILESIZE={}MB", cli.filesize);
    println!("RECORD_SIZE={}B", RECORD_SIZE);
    println!("HASH_SIZE={}B", HASH_SIZE);
    println!("NONCE_SIZE={}B", NONCE_SIZE);
    println!("BUCKET_SIZE={}MB", memory_limit_in_bytes);
}

// Function to generate the Blake3 hash for the given nonce
fn generate_hash(nonce: &[u8]) -> [u8; HASH_SIZE] {
    let hash = blake3::hash(nonce);
    return hash.as_bytes()[0..10].try_into().unwrap();
}

// Function to write the Records in memory to the file
fn write_to_file(records: &Vec<Record>, num_threads: usize) {
    // Create a Thread Pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    // Open the file and create a buffered writer for it
    let file = File::options().append(true).create(true).write(true).open("data.bin").expect("Couldn't create file"); 
    let mut writer = BufWriter::new(&file);  

    // Closure to write the hashes in memory to the file in the created Thread Pool
    pool.install(|| {
        records
        .into_iter()
        .for_each(|record| { 
            writer.write_all(&record.hash).expect("Failed to write hash to file");
            writer.write_all(&record.nonce).expect("Failed to write nonce to file");
        });
    });
}

// Function to Generate Records
fn generate_records(n_iterations: i64, num_threads: usize) ->Vec <Record> {
    // Create a Thread Pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    // CLosure to generate the hashes in parallel in the created Thread Pool
    pool.install(|| {
        (1..=n_iterations)
        .into_par_iter()
        .map(|_| {
            // Generate a random number as u64
            let mut rng = rand::thread_rng();
            let random_num: u64 = rng.gen::<u64>();
            // Convert the u64 into a u8 byte array
            let random_num_be_bytes = random_num.to_be_bytes();
            // Extract the first 6 bytes of the byte array to get the nonce
            let nonce = [random_num_be_bytes[0], random_num_be_bytes[1], random_num_be_bytes[2], random_num_be_bytes[3], random_num_be_bytes[4], random_num_be_bytes[5]];
            // Generate the hash using the nonce
            let hash: [u8; 10] = generate_hash(&nonce);
            // Create a new Record containing the hash and the nonce
            Record::new(hash, nonce)
        })
        .collect()
    })
}

fn sort_hashes(mut records: Vec<Record>, num_threads: usize) -> Vec<Record> {
    // Create a Thread Pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    // Sort the generated records in place
    pool.install(|| {
        records.par_sort_unstable_by(|a, b| (a.hash).cmp(&b.hash));
    });

    return records;
}

fn main() {
    // Parsing Command Line Arguments
    let cli = Cli::parse();
    let memory_limit: i64 = cli.memory_size;
    let memory_limit_in_bytes: i64 = memory_limit * (2i64.pow(20));
    let n_iterations = memory_limit_in_bytes/(RECORD_SIZE as i64);
    let total_iterations = cli.filesize/memory_limit;

    // Remove the output file, if any
    let _ = fs::remove_file(&cli.filename);

    // Check if the program is in debug mode or not
    let debug = cli.debug.as_deref().unwrap_or("false");
    if debug == "true" {
        
        let bar = ProgressBar::new(total_iterations as u64);
        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [ETA:{eta_precise}]")
            .unwrap()
            .progress_chars("##-"));
        print_args(&cli, &memory_limit);
        let total_start = Instant::now();
        let mut hash_duration: Duration = Duration::from_millis(0);
        let mut sort_duration: Duration = Duration::from_millis(0);
        let mut file_duration: Duration = Duration::from_millis(0);
        
        for _ in 0..total_iterations {
            bar.inc(1);
            // Hashing stage
            bar.clone().with_message("Hashing");
            let _hash_start = Instant::now();
            let records = generate_records(n_iterations, cli.num_threads_hash);
            let _hash_duration = _hash_start.elapsed();
            hash_duration = hash_duration + _hash_duration;
            let current_mem = PEAK_ALLOC.current_usage_as_mb();
	        println!("Hashing used {} MB of RAM.", current_mem);

            // Sorting Stage
            bar.clone().with_message("Sorting");
            let _sort_start = Instant::now();
            let records = sort_hashes(records, cli.num_threads_sort);
            let _sort_duration = _sort_start.elapsed();
            sort_duration = sort_duration + _sort_duration;
            let current_mem = PEAK_ALLOC.current_usage_as_mb();
	        println!("Sorting used {} MB of RAM.", current_mem);

            // Writing Stage
            bar.clone().with_message("Writing");
            let _file_start = Instant::now();
            write_to_file(&records, cli.num_threads_write);
            let _file_duration = _file_start.elapsed();
            file_duration = file_duration + _file_duration;
            let current_mem = PEAK_ALLOC.current_usage_as_mb();
	        println!("Writing used {} MB of RAM.", current_mem);
        }

        // Print the final output
        let total_duration = total_start.elapsed();
        println!("hashgen t{:?} o{:?} i{:?} m{:?} s{:?} {:?} {:?} {:?} {:?}", cli.num_threads_hash, cli.num_threads_sort, cli.num_threads_write, cli.memory_size, cli.filesize, hash_duration, sort_duration, file_duration, total_duration);
        let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
	    println!("The max amount that was used {} MB", peak_mem);
    } else {
        // Initialize the timers
        let total_start = Instant::now();
        let mut hash_duration: Duration = Duration::from_millis(0);
        let mut sort_duration: Duration = Duration::from_millis(0);
        let mut file_duration: Duration = Duration::from_millis(0);
        
        for _ in 0..total_iterations {
            // Hashing stage
            let _hash_start = Instant::now();
            let records = generate_records(n_iterations, cli.num_threads_hash);
            let _hash_duration = _hash_start.elapsed();
            hash_duration = hash_duration + _hash_duration;

            // Sorting Stage
            let _sort_start = Instant::now();
            let records = sort_hashes(records, cli.num_threads_sort);
            let _sort_duration = _sort_start.elapsed();
            sort_duration = sort_duration + _sort_duration;

            // Writing Stage
            let _file_start = Instant::now();
            write_to_file(&records, cli.num_threads_write);
            let _file_duration = _file_start.elapsed();
            file_duration = file_duration + _file_duration;
        }

        // Print the final output
        let total_duration = total_start.elapsed();
        println!("hashgen t{:?} o{:?} i{:?} m{:?} s{:?} {:?} {:?} {:?} {:?}", cli.num_threads_hash, cli.num_threads_sort, cli.num_threads_write, cli.memory_size, cli.filesize, hash_duration, sort_duration, file_duration, total_duration);
    }
}
