use std::{
    cmp::Ordering, thread, time::Duration
};
use rand::{self, prelude::*};
use soloud::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();

    let oc_spray = include_bytes!("oc_spray.wav");
    let pistol = include_bytes!("pistol.wav");
    let batong = include_bytes!("batong.wav");

    let mut counts = [0, 0, 0];
    let mut to_play: u8 = 9;
    loop {
        // Dirty ass solution
        if to_play == 0 {
            to_play = rng.gen_range(1..=2);
        } else if to_play == 1 {
            if rng.gen() {
                to_play = 0
            } else {
                to_play = 2
            }
        } else if to_play == 2 {
            to_play = rng.gen_range(0..=1);
        } else {
            to_play = rng.gen_range(0..=2);
        }

        match to_play.cmp(&1) {
            Ordering::Less => {
                wav.load_mem(oc_spray).unwrap();
                counts[0] += 1;
            }
            Ordering::Equal => {
                wav.load_mem(pistol).unwrap();
                counts[1] += 1;
            }
            Ordering::Greater => {
                wav.load_mem(batong).unwrap();
                counts[2] += 1;
            }
        }

        // Sleep between 3s -> 10s
        let slept = Duration::from_millis(100 * rng.gen_range(30..=100));
        thread::sleep(slept);

        sl.play(&wav);
        while sl.voice_count() > 0 {
            thread::sleep(Duration::from_millis(100));
        }
        println!(
            "OC-Spray: {}, Pistol: {}, Batong: {}, Time slept: {}s",
            &counts[0], &counts[1], &counts[2], &slept.as_secs_f64()
        );
    }
}
