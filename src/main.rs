use std::{i16, path::Path};

const SAMPLE_RATE: usize = 44100;

fn gain(inp: &[f64], gain: f64) -> Vec<f64> {
    inp.into_iter().map(|f| f * gain).collect()
}

fn delay(inp: &[f64], delay: f64) -> Vec<f64> {
    let samples = (delay * SAMPLE_RATE as f64) as usize;
    let mut ret = vec![0f64; samples];
    ret.extend(inp);
    ret
}

fn combine(inps: &[Vec<f64>]) -> Vec<f64> {
    let max_len = inps.iter().map(|l| l.len()).max().unwrap_or_default();
    let mut ret = vec![0f64; max_len];
    for l in inps.iter() {
        for (i, s) in l.iter().enumerate() {
            ret[i] += *s;
        }
    }
    ret
}

fn reverb(inp: &[f64]) -> Vec<f64> {
    let mut delays = vec![];
    let mut atten = 0.1f64;
    for i in 0..10 {
        let delayed = delay(&gain(&inp, atten), i as f64 / 5f64);
        atten *= 0.8f64;
        delays.push(delayed);
    }
    combine(&delays)
}

fn read_samples<P: AsRef<Path>>(path: P) -> Result<Vec<f64>, std::io::Error> {
    let bytes = std::fs::read(path)?;
    Ok(bytes
        .chunks(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]) as f64 / i16::MAX as f64)
        .collect::<Vec<_>>())
}

fn write_samples<P: AsRef<Path>>(path: P, samples: &[f64]) -> Result<(), std::io::Error> {
    let bytes = samples
        .into_iter()
        .map(|i| ((i * (i16::MAX as f64)) as i16).to_le_bytes())
        .flatten()
        .collect::<Vec<_>>();
    std::fs::write(path, bytes)?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let samples = read_samples("tuyo.wav")?;
    let reverbed = reverb(&samples);
    write_samples("out.wav", &reverbed)?;
    Ok(())
}
