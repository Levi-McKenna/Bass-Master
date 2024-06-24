use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use ringbuf::*;
use crossbeam_channel::{bounded, Receiver};
use std::thread;

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
                            // if out of bounds... don't count
                            if estimate.frequency > 20. && estimate.frequency < 88. {
                                tx.send(estimate.frequency).unwrap();
                                println!("Estimated Frequency: {}", estimate.frequency);
                            }
                        }
                        consumer.clear();
                    } else {
                        producer.push_slice(input);
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
    println!("Reading Audio Stream");
}
