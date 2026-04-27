# Modem

In this project I created a detector for bits in audio signals that follow the Bell 103 modem protocol. The program takes in a wav file (48 kilosample, 16 bit mono-channel) and detects answer side of the Bell 103 modem protocol. Input and Output files can be specified.

## Running

To compile and run on base files simply use the following command

`cargo run --release`

## Options

The program recognizes a few helpful options. Use `--help` for more details.

- `input <file_path>` - specify input file location (default: `./input/message.wav`)
- `output <file_path>` - specify output file path (default: `./message.txt`)
- `test1` - run program with test input file 1
- `test2` - run program with test input file 2
- debug - run program with debug output

### example

`cargo run --release -- --input ~/docs/my_input.wav --output ~/docs/my_output.txt --debug`
