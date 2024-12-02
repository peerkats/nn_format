mod math;
mod file_builder;
use file_builder::{Layer, Model};

fn main() -> std::io::Result<()> {
    // Create some dummy data for the model
    let model = Model {
        layers: vec![
            Layer1 {
                weights: vec![vec![0.1, 0.2], vec![0.3, 0.4], vec![0.5, 0.5]],
                biases: vec![0.1, 0.2],
            }
        ],
    };

    // Save the model to a binary file
    model.save("model.bin")?;

    // Load the model from the binary file
    let loaded_model = Model::load("model.bin")?;

    // Print the loaded model
    println!("Loaded model: {:?}", loaded_model);

    Ok(())
}