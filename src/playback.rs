use cpal::{Sample, traits::DeviceTrait};
use crate::errors;

pub(crate) fn audio_playback<T: Sample>(data: &mut [T], callback_info: &cpal::OutputCallbackInfo) {
    ()
}

pub(crate) fn get_stream(device: &cpal::platform::Device, conf: &cpal::StreamConfig, form: &cpal::SampleFormat) -> eyre::Result<cpal::Stream> {
    let m = match form { 
        cpal::SampleFormat::I16 => device.build_output_stream(
            conf, 
            audio_playback::<i16>, 
            |err| eprintln!("{}", err), 
            None
        ),
        cpal::SampleFormat::F32 => device.build_output_stream(
            conf, 
            audio_playback::<f32>, 
            |err| eprintln!("{}", err), 
            None
        ),
        x => Err(errors::AutumnErrors::SampleTypeNotSupported(*x))?
    }?;
    Ok(m)
}