// src/main.rs

use std::{path::PathBuf, fs::File};
use image::{ImageFormat, DynamicImage};
use clap::Parser;
use anyhow::{Result, Context};
use mozjpeg::{Compress, ColorSpace, ScanOpt, TrellisOpt};

// --- CLI Structure ---

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input image file path
    #[arg(short, long)]
    input: PathBuf,

    // Output file path
    #[arg(short, long)]
    output: PathBuf,

    // Compression quality (0-100)
    #[arg(short, long, default_value_t = 85)]
    quality: u8,
}

// --- Main Logic ---
fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Check quality range
    if args.quality == 0 || args.quality > 100 {
        return Err(anyhow::anyhow!("Quality must be between 1 and 100."));
    }

    // 1. Load the input image
    let img = image::open(&args.input)
        .with_context(|| format!("Failed to open input file: {}", args.input.display()))?;

    // 2. Run the compression process
    let compressed_data = compress_image_mozjpeg(&img, args.quality)?;

    // 3. Save the compressed data to the output file
    File::create(&args.output)
        .with_context(|| format!("Failed to create output file: {}", args.output.display()))?
        .write_all(&compressed_data)
        .with_context(|| format!("Failed to write compressed data."))?;

    println!("✅ Compression complete!");
    println!("Output saved to: {}", args.output.display());

    Ok(())
}

// --- MozJPEG Compression Function ---
fn compress_image_mozjpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>> {
    // Convert to RGB, which is optimal for MozJPEG
    let rgb_img = img.to_rgb8();
    let width = rgb_img.width() as usize;
    let height = rgb_img.height() as usize;
    let data = rgb_img.into_raw();

    // Initialize the MozJPEG compressor with high optimization settings
    let mut compressor = Compress::new(ColorSpace::JCS_RGB)?;
    
    // Set parameters for the output JPEG
    compressor.set_quality(quality as f32);
    compressor.set_size(width, height);
    
    // Advanced optimization settings provided by MozJPEG for maximum savings
    compressor.set_optimize_coding(true);
    compressor.set_progressive_mode(true);
    compressor.set_scan_optimization(ScanOpt::Fast); 
    compressor.set_trellis_optimization(TrellisOpt::Default);

    // Start compression
    compressor.start_compress()?;
    compressor.write_scanlines(&data)?;
    compressor.finish_compress()?;

    // Retrieve the final JPEG byte buffer
    let compressed_data = compressor.data_out()?;
    
    Ok(compressed_data)
}