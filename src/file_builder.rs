use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use crate::math::Matrix;

#[derive(Debug)]
pub struct Layer {
    pub weights: Vec<Vec<f32>>,
    pub biases: Vec<f32>,
}

#[derive(Debug)]
pub struct Model {
    pub layers: Vec<Layer>,
}

impl Model {
    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write the number of layers
        writer.write_all(&(self.layers.len() as u32).to_le_bytes())?;

        for layer in &self.layers {
            // Write the dimensions of the weights matrix
            let rows = layer.weights.len() as u32;
            let cols = layer.weights[0].len() as u32;
            writer.write_all(&rows.to_le_bytes())?;
            writer.write_all(&cols.to_le_bytes())?;

            // Write the weights
            for row in &layer.weights {
                for &weight in row {
                    writer.write_all(&weight.to_le_bytes())?;
                }
            }

            // Write the biases
            writer.write_all(&(layer.biases.len() as u32).to_le_bytes())?;
            for &bias in &layer.biases {
                writer.write_all(&bias.to_le_bytes())?;
            }
        }

        Ok(())
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read the number of layers
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let num_layers = u32::from_le_bytes(buffer) as usize;

        let mut layers = Vec::with_capacity(num_layers);

        for _ in 0..num_layers {
            // Read the dimensions of the weights matrix
            reader.read_exact(&mut buffer)?;
            let rows = u32::from_le_bytes(buffer) as usize;
            reader.read_exact(&mut buffer)?;
            let cols = u32::from_le_bytes(buffer) as usize;

            // Read the weights
            let mut weights = Vec::with_capacity(rows);
            for _ in 0..rows {
                let mut row = Vec::with_capacity(cols);
                for _ in 0..cols {
                    let mut weight_buffer = [0u8; 4];
                    reader.read_exact(&mut weight_buffer)?;
                    row.push(f32::from_le_bytes(weight_buffer));
                }
                weights.push(row);
            }

            // Read the biases
            reader.read_exact(&mut buffer)?;
            let num_biases = u32::from_le_bytes(buffer) as usize;
            let mut biases = Vec::with_capacity(num_biases);
            for _ in 0..num_biases {
                let mut bias_buffer = [0u8; 4];
                reader.read_exact(&mut bias_buffer)?;
                biases.push(f32::from_le_bytes(bias_buffer));
            }

            layers.push(Layer { weights, biases });
        }

        Ok(Model { layers })
    }
}