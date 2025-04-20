mod cli;
mod processing;
mod utils;

use clap::Parser;
use cli::CliArgs;
use utils::infer_output_path;

fn main() {
    let args = CliArgs::parse();

    let output_path = infer_output_path(&args.input, &args.output);

    println!("📸 Input: {}", args.input);
    println!("💾 Output: {}", output_path.display());

    match processing::process_image(&args.input, output_path.to_str().unwrap()) {
        Ok(_) => println!("✅ Silhouette saved to {}", output_path.display()),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
