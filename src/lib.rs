use nih_plug::prelude::*;
use std::sync::Arc;

struct AudioEqPlugin {
    params: Arc<AudioEqPluginParams>,
}

#[derive(Params)]
struct AudioEqPluginParams {
    #[id = "frequency"]
    pub frequency: FloatParam,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "q"]
    pub q: FloatParam,
}

impl Default for AudioEqPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(AudioEqPluginParams::default()),
        }
    }
}

impl Default for AudioEqPluginParams {
    fn default() -> Self {
        fn parse_f32(input: &str) -> Option<f32> {
            input.parse().ok()
        }

        Self {
            frequency: FloatParam::new(
                "Frequency",
                1000.0,
                FloatRange::Linear { min: 20.0, max: 20000.0 },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_string_to_value(Arc::new(parse_f32)),

            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear { min: -24.0, max: 24.0 },
            )
            .with_unit(" dB")
            .with_step_size(0.1)
            .with_value_to_string(formatters::v2s_f32_rounded(1))
            .with_string_to_value(Arc::new(parse_f32)),

            q: FloatParam::new(
                "Q",
                1.0,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 10.0,
                    factor: 0.5,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_string_to_value(Arc::new(parse_f32)),
        }
    }
}

impl Plugin for AudioEqPlugin {
    const NAME: &'static str = "Audio EQ Plugin";
    const VENDOR: &'static str = "BESTEQEVER123";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for AudioEqPlugin {
    const CLAP_ID: &'static str = "com.example.audio-eq-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple parametric EQ plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for AudioEqPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"EqPlug1234567890";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(AudioEqPlugin);
nih_export_vst3!(AudioEqPlugin);
