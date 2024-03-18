use cpal::*;

pub fn read_audio_samples() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Default input device not selected");
    let err_fn = |err| eprintln!("an error occured on the input audio stream: {}", err);
    let mut supported_input_configs_range = device.supported_input_configs()
        .expect("Couldn't query for input configs");
    let supported_input_config = supported_input_configs_range.next()
        .expect("No supported config")
        .with_max_sample_rate();
    let input_stream = device.build_input_stream(
        &supported_input_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            
        },
        err_fn,
        None
    )
}
