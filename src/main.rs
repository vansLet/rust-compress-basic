use rust_compress_basic::{cli, compress_file, decompress_file};
fn main() {
    let cli = cli::CliArguments::new().unwrap();
    match &cli.type_encrypt {
        cli::OptionCommands::Compress => compress_file(&cli.from_path, &cli.to_path).unwrap(),
        cli::OptionCommands::Decompress => decompress_file(&cli.from_path, &cli.to_path).unwrap(),
    }
    return;
}
