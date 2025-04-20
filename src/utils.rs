use std::path::{Path, PathBuf};

use image::GrayImage;
use ndarray::Array2;

/// A grayscale matrix of pixel values (0.0 to 255.0)
#[derive(Debug, Clone)]
pub struct GrayMatrix(pub Array2<f32>);

/// A wrapper around a processed silhouette image
#[derive(Debug, Clone)]
pub struct Silhouette(pub GrayImage);

/// If the output arg has no extension, copy the input's extension
pub fn infer_output_path(input: &str, output: &str) -> PathBuf {
    let input_ext: &str = Path::new(input)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png");

    let mut output_path: PathBuf = PathBuf::from(output);

    if output_path.extension().is_none() {
        output_path.set_extension(input_ext);
    }

    output_path
}
