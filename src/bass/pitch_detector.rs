use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use ringbuf::*;
use ringbuf::ring_buffer::{RbRead, RbRef, RbWrite};
use crossbeam_channel::{bounded, Receiver};
use std::thread;
use std::time::Duration;
use crate::{NoteCollision};

/* pub fn setup_audiostream() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("failed to find input device");
    let config = device.default_input_config().unwrap();
    println!("{:?}", config.buffer_size());
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(device, config.into()),
        cpal::SampleFormat::I16 => run::<i16>(device, config.into()),
        cpal::SampleFormat::U16 => run::<u16>(device, config.into()),
        _ => todo!(),
    }
} */

/* #[derive(Event)]
pub struct ChordFret {
    chord: String,
    fret: i8,
}

impl ChordFret {
    pub fn new(chord: &str, fret: i8) -> ChordFret {
        ChordFret {
            chord: chord.to_string(),
            fret: fret
        }
    }
} */

#[derive(Resource, Deref)]
pub struct StreamReceiver(Receiver<f64>);

pub fn read_audiostream(
    mut commands: Commands,
) {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("failed to find input device");
    let config = device.default_input_config().unwrap();

    let (tx, rx) = bounded::<f64>(1);
    thread::spawn(move || {
        // ring buffer initialization
        let ring_buffer = HeapRb::<f32>::new(8192);
        let (mut producer, mut consumer) = ring_buffer.split();

        let err_fn = |err| println!("{}", err);
        let stream = device
            .build_input_stream(
                &config.into(),
                move |input, _| {
                    const SAMPLE_RATE: usize = 44100;
                    const SIZE: usize = 8192;
                    const PADDING: usize = SIZE / 2;
                    const POWER_THRESHOLD: f64 = 10.;
                    const CLARITY_THRESHOLD: f64 = 0.6;
                    
                    if consumer.is_full() {
                        let f64_vals: Vec<f64> = consumer.iter().map(|x| *x as f64).collect();
                        let mut detector = McLeodDetector::new(SIZE, PADDING);
                        if let Some(estimate) = detector.get_pitch(&f64_vals, SAMPLE_RATE, POWER_THRESHOLD, CLARITY_THRESHOLD) {
                            tx.send(estimate.frequency).unwrap();
/*                                     println!("Chord -> A, Fret -> {}", i); */
                            println!("Estimated Frequency: {}", estimate.frequency);
                        } else {
                            tx.send(-1.).unwrap();
                            println!("Estimated Frequency: -1");
                        }
                            consumer.clear();
                    } else {
                        let index_num = producer.push_slice(input);
                    }
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {}
    });

    commands.insert_resource(StreamReceiver(rx));
}

