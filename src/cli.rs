use clap::Parser;

/// CLI options for silhouette tracker
#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Input image path
    #[arg(short, long)]
    pub input: String,

    /// Output image path (optional, auto-detects extension from input)
    #[arg(short, long, default_value = "output_silhouette")]
    pub output: String,
}
