// USD support can wait
extern crate hound;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("**********");
    println!("* BANNER *");
    println!("**********");

    let sample_rate = 48_000;
    let duration = 2.0;
    let num_samples = (sample_rate as f32 * duration) as usize;
    
    let frequency = 240.0;
    let buffer: Vec<f32> = (0..num_samples)
        .map(|x| (x as f32 * frequency * 2.0 * std::f32::consts::PI / sample_rate as f32).sin() / 1.5)
        .collect();

    let mut writer = hound::WavWriter::create("output.wav", hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    })?;

    for sample in buffer {
        writer.write_sample(sample)?;
    }
    
    writer.finalize()?;

    return Ok(());
    // rust is hard
}
