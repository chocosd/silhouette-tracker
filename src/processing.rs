use crate::utils::{GrayMatrix, Silhouette};
use image::{GrayImage, Luma};
use ndarray::Array2;
use std::error::Error;

pub fn process_image(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let raw = image::open(input_path)?.to_luma8();
    let gray_matrix = convert_to_matrix(&raw);
    let silhouette = sobel_edge_detection(&gray_matrix);

    silhouette.0.save(output_path)?;
    Ok(())
}

/// Convert image to matrix of grayscale f32 values
fn convert_to_matrix(img: &GrayImage) -> GrayMatrix {
    let (width, height) = img.dimensions();
    let mut matrix = Array2::<f32>::zeros((height as usize, width as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        matrix[[y as usize, x as usize]] = pixel[0] as f32;
    }

    GrayMatrix(matrix)
}

/// Apply Sobel filter to get silhouette from grayscale matrix
fn sobel_edge_detection(input: &GrayMatrix) -> Silhouette {
    let (height, width) = (input.0.nrows(), input.0.ncols());
    let mut output = GrayImage::new(width as u32, height as u32);

    let gx = [[-1f32, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
    let gy = [[-1f32, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;

            for j in 0..3 {
                for i in 0..3 {
                    let px = input.0[[y + j - 1, x + i - 1]];
                    sum_x += px * gx[j][i];
                    sum_y += px * gy[j][i];
                }
            }

            let magnitude = (sum_x.powi(2) + sum_y.powi(2)).sqrt().min(255.0);
            output.put_pixel(x as u32, y as u32, Luma([magnitude as u8]));
        }
    }

    Silhouette(output)
}
