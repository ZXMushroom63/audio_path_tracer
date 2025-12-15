// USD support can wait
extern crate hound;
use std::string::String;
use std::f32;
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}
struct Pixel {
    pos: Vec3,
    forward: Vec3
}
impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Self { x, y, z };
    }
    fn to_string(&self) -> String {
        return format!("{} {} {}", &self.x, &self.y, &self.z);
    }
    fn length_square(&self) -> f32 {
        return (&self.x + &self.y + &self.z).abs();
    }
    fn length(&self) -> f32 {
        return self.length_square().sqrt();
    }
    fn add(&self, vec: &Vec3) -> Vec3 {
        return Vec3::new(&self.x + &vec.x, &self.y + &vec.y, &self.z + &vec.z);
    }
    fn subt(&self, vec: &Vec3) -> Vec3 {
        return Vec3::new(&self.x - &vec.x, &self.y - &vec.y, &self.z - &vec.z);
    }
    fn mult(&self, vec: &Vec3) -> Vec3 {
        return Vec3::new(&self.x * &vec.x, &self.y * &vec.y, &self.z * &vec.z);
    }
    fn scale(&self, s: f32) -> Vec3 {
        return Vec3::new(&self.x * s, &self.y * s, &self.z * s);
    }
    fn normalise(&self) -> Vec3 {
        let max: f32 = self.x.max(self.y.max(self.z));
        return Vec3::new(self.x / max, self.y / max, self.z / max);
    }
}

fn signed_distance_function(pos: Vec3) -> f32 {
    // a simple sphere
    // radius of 2
    return pos.length() - 2.0;
}

fn cosD(x: f32) -> f32 {
    return ((3.14 / 180.0) * x).cos(); 
}

fn sinD(x: f32) -> f32 {
    return ((3.14 / 180.0) * x).sin(); 
}
fn main() {
    let cPitch: f32 = 0.0;
    let cYaw: f32 = 0.0;
    let forwardVector = Vec3::new(
        cosD(cPitch) * cosD(cYaw), 
        cosD(cPitch) * sinD(cYaw), 
        sinD(cPitch));
    let cameraPos = Vec3::new(0.0, 0.0, 20.0);
    let pixelSize = Vec3::new(1.0, 1.0, 0.0);
    let widthPx = 80;
    let heightPx = 60;
    let focalLength: f32 = 2.0;
    let cameraProjectPos: Vec3 = forwardVector.scale(-focalLength).add(&cameraPos);
    let mut screen: Vec<Vec<Pixel>> = Vec::new();
    for x in (0..widthPx) {
        screen.push(Vec::new());
        for y in (0..heightPx) {
            screen[x].push(Pixel {
                pos: Vec3::new(
                    (x - widthPx / 2) as f32 * pixelSize.x,
                    (y - heightPx / 2) as f32 * pixelSize.y,
                    0.0
                ).add(&cameraPos),
                forward: Vec3::new(0.0,0.0,0.0)
            });
            screen[x][y].forward = screen[x][y].pos.subt(&cameraProjectPos).normalise();
        }
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let sample_rate = 48_000;
//     let duration = 2.0;
//     let num_samples = (sample_rate as f32 * duration) as usize;
    
    
//     let frequency = 440.0;
//     let mut buffer: Vec<f32> = vec![0.0; num_samples];

//     let mut index: i32 = 0;
//     for sample in &mut buffer {
//         *sample += (((index as f32) / (sample_rate as f32)) * std::f32::consts::PI * frequency * 2.0).sin();
//         index += 1;
//     }
//     println!("buffer length = {}", buffer.len());

//     let mut writer = hound::WavWriter::create("output.wav", hound::WavSpec {
//         channels: 1,
//         sample_rate,
//         bits_per_sample: 32,
//         sample_format: hound::SampleFormat::Float,
//     })?;

//     for sample in buffer {
//         writer.write_sample(sample)?;
//     }
    
//     writer.finalize()?;

//     return Ok(());
//     // rust is hard
// }


/*
The Plan:
ASCII art thing idfk
*/