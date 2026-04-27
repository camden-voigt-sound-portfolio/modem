use clap::Parser;
use hound::WavReader;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

/// Program to decode ascii text messages encoded as audio use the Bell 103 protocol.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Location of input file
    #[arg(short, long, default_value_t = String::from("input/message.wav"))]
    input: String,

    /// Location of output file
    #[arg(short, long, default_value_t = String::from("message.txt"))]
    output: String,

    /// Use test file 1
    #[arg(long)]
    test1: bool,

    /// Use test file 2
    #[arg(long)]
    test2: bool,

    /// toggle debug mode ON
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    // Hanlde Args
    let args = Args::parse();
    let mut input_file_name = args.input.as_str();
    if args.test1 {
        input_file_name = "input/test1.wav";
    } else if args.test2 {
        input_file_name = "input/test2.wav";
    }
    let output_file_name = args.output.as_str();
    let debug = args.debug;

    // Get blocks of samples
    let mut input = WavReader::open(input_file_name).unwrap();

    let num_blocks: usize = input.len() as usize / 160;
    if debug {
        println!("num_samples: {}\n num_blocks: {}", input.len(), num_blocks);
    }
    let mut blocks: Vec<Vec<i16>> = Vec::with_capacity(num_blocks);
    for _ in 0..num_blocks {
        blocks.push(Vec::new())
    }
    for (i, sample) in input.samples::<i16>().enumerate() {
        let s = sample.unwrap();
        let block = i / 160;
        blocks[block].push(s);
    }

    // Get bits by doing power
    let mut bits: Vec<i16> = Vec::new();
    if debug {
        println!("Power Diffs");
        println!("-------------");
    }
    for (i, block) in blocks.iter().enumerate() {
        let mark_power = tone_power(block, 2225.0);
        let space_power = tone_power(block, 2025.0);
        let diff = mark_power - space_power;
        if debug {
            println!("{}: {} - {} = {}", i, mark_power, space_power, diff);
        }

        if diff < 0.0 {
            bits.push(0);
        } else if diff > 0.0 {
            bits.push(1);
        } else {
            panic!("this is bad!");
        }
    }

    // convert bits to characters
    if debug {
        println!("Bytes");
        println!("----------");
    }
    let mut message = String::from("");
    let mut bs = bits.as_slice();
    for _ in 0..=((num_blocks / 10) - 1) {
        let (byte, bits) = bs.split_at(10);
        bs = bits;
        assert!(byte[0] == 0);
        assert!(byte[9] == 1);
        let mut num: u8 = 0;
        // Go through actual data bits in reverse order since they come in LSB first
        for bit in (1..=8).rev() {
            num <<= 1;
            num |= byte[bit] as u8;
        }
        if debug {
            println!("{:?}", byte);
            println!("num - {:?}", num);
        }
        message.push(num as char);
    }

    // Write to file
    println!(
        "Final Message (saved in {}) - {}",
        output_file_name, message
    );
    let mut file = File::create(output_file_name).unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

// Calculate i/q power for a give set of samples and frequency
fn tone_power(samples: &[i16], freq: f32) -> f32 {
    let mut i = 0.0;
    let mut q = 0.0;
    for (k, sample) in samples.iter().enumerate() {
        let angle = 2.0 * PI * freq * (k as f32 / 48000.0);
        i += (*sample as f32) * f32::cos(angle);
        q += (*sample as f32) * f32::sin(angle);
    }
    i * i + q * q
}
