mod processing;
mod utils;

fn main() {
    println!("📸 Starting silhouette simulation...");

    let input_path = "test_image_1.jpg";
    let output_path = "output_silhouette.png";

    if let Err(e) = processing::process_image(input_path, output_path) {
        eprintln!("❌ Error: {}", e);
    } else {
        println!("✅ Silhouette saved to {}", output_path);
    }
}
