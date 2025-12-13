// USD support can wait
extern crate hound;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("**********");
    println!("* BANNER *");
    println!("**********");

    let sample_rate = 48_000;
    let duration = 2.0;
    let num_samples = (sample_rate as f32 * duration) as usize;
    
    let frequency = 440.0;
    let mut buffer: Vec<f32> = vec![0.0; num_samples];

    let mut index: i32 = 0;
    for sample in &mut buffer {
        *sample += (((index as f32) / (sample_rate as f32)) * std::f32::consts::PI * frequency).sin();
        index += 1;
    }
    println!("buffer length = {}", buffer.len());

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
