use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use turbo_quant::{PolarQuantizer, QjlQuantizer, TurboQuantizer};

pub struct TurboProcessor {
    dimension: usize,
    seed: u64,
    turbo: TurboQuantizer,
}

impl TurboProcessor {
    pub fn new(dimension: usize, seed: u64) -> Result<Self> {
        let turbo = TurboQuantizer::new(dimension, 8, dimension / 4, seed)
            .context("Failed to create TurboQuantizer")?;

        Ok(Self {
            dimension,
            seed,
            turbo,
        })
    }

    pub fn text_to_vector(&self, text: &str) -> Vec<f32> {
        let mut vector = vec![0.0f32; self.dimension];

        for (i, c) in text.chars().enumerate() {
            if i >= self.dimension {
                break;
            }
            vector[i] = c as u32 as f32 / 255.0;
        }

        for (i, word) in text.split_whitespace().enumerate() {
            let idx = self.dimension / 2 + (i % (self.dimension / 2));
            if idx < self.dimension {
                let mut hasher = DefaultHasher::new();
                word.hash(&mut hasher);
                vector[idx] = (hasher.finish() as f32 % 1000.0) / 1000.0;
            }
        }

        let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for v in &mut vector {
                *v /= magnitude;
            }
        }

        vector
    }

    pub fn enhance_response(&self, text: &str) -> Result<EnhanceResult> {
        let original_vector = self.text_to_vector(text);

        let code = self.turbo.encode(&original_vector)?;

        let approximate = self.turbo.decode_approximate(&code)?;

        let mut enhanced_chars: Vec<char> = Vec::new();
        for (i, &val) in approximate.iter().enumerate() {
            if i < text.len() {
                let scaled = ((val * 255.0).round() as u8).max(32).min(126);
                enhanced_chars.push(scaled as char);
            }
        }

        let enhanced = enhanced_chars.iter().collect::<String>();
        let enhanced_length = enhanced.len();
        let original_length = text.len();
        let compression_ratio = code.compression_ratio();
        let encoded_bytes = code.encoded_bytes();

        Ok(EnhanceResult {
            original: text.to_string(),
            enhanced,
            original_length,
            enhanced_length,
            compression_ratio,
            encoded_bytes,
        })
    }

    pub fn polar_enhance(&self, text: &str) -> Result<EnhanceResult> {
        let pq = PolarQuantizer::new(self.dimension, 8, self.seed)?;
        let vector = self.text_to_vector(text);

        let code = pq.encode(&vector)?;
        let decoded = pq.decode(&code)?;

        let mut enhanced_chars: Vec<char> = Vec::new();
        for (i, &val) in decoded.iter().enumerate() {
            if i < text.len() {
                let scaled = ((val * 255.0).round() as u8).max(32).min(126);
                enhanced_chars.push(scaled as char);
            }
        }

        let enhanced = enhanced_chars.iter().collect::<String>();
        let enhanced_length = enhanced.len();
        let original_length = text.len();
        let compression_ratio = code.encoded_bytes() as f32 / (self.dimension * 4) as f32;
        let encoded_bytes = code.encoded_bytes();

        Ok(EnhanceResult {
            original: text.to_string(),
            enhanced,
            original_length,
            enhanced_length,
            compression_ratio,
            encoded_bytes,
        })
    }
}

#[derive(Debug)]
pub struct EnhanceResult {
    pub original: String,
    pub enhanced: String,
    pub original_length: usize,
    pub enhanced_length: usize,
    pub compression_ratio: f32,
    pub encoded_bytes: usize,
}

impl Default for TurboProcessor {
    fn default() -> Self {
        Self::new(256, 42).unwrap()
    }
}
