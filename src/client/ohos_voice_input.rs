use base::message_proto::{AudioFormat, AudioFrame, Message, Misc};
use magnum_opus::{Application::LowDelay, Channels::Mono, Encoder};

const SAMPLE_RATE: u32 = 48_000;
const SAMPLES_PER_FRAME: usize = 960;
const MAX_INPUT_BYTES: usize = 32 * 1024;

pub(super) struct OhosVoiceInput {
    encoder: Encoder,
    pending: Vec<u8>,
}

impl OhosVoiceInput {
    pub fn new() -> hbb_common::ResultType<Self> {
        Ok(Self {
            encoder: Encoder::new(SAMPLE_RATE, Mono, LowDelay)?,
            pending: Vec::with_capacity(SAMPLES_PER_FRAME * 2),
        })
    }

    pub fn format_message() -> Message {
        let mut misc = Misc::new();
        misc.set_audio_format(AudioFormat {
            sample_rate: SAMPLE_RATE,
            channels: 1,
            ..Default::default()
        });
        let mut message = Message::new();
        message.set_misc(misc);
        message
    }

    pub fn encode(&mut self, pcm: &[u8]) -> Vec<Message> {
        if pcm.is_empty() || pcm.len() > MAX_INPUT_BYTES || pcm.len() % 2 != 0 {
            return Vec::new();
        }
        self.pending.extend_from_slice(pcm);
        let frame_bytes = SAMPLES_PER_FRAME * 2;
        let mut messages = Vec::new();
        while self.pending.len() >= frame_bytes {
            let samples: Vec<f32> = self.pending[..frame_bytes]
                .chunks_exact(2)
                .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]) as f32 / 32768.0)
                .collect();
            self.pending.drain(..frame_bytes);
            match self.encoder.encode_vec_float(&samples, 4000) {
                Ok(encoded) => {
                    let mut message = Message::new();
                    message.set_audio_frame(AudioFrame {
                        data: encoded.into(),
                        ..Default::default()
                    });
                    messages.push(message);
                }
                Err(error) => {
                    hbb_common::log::warn!("Failed to encode OHOS voice input: {error:?}");
                }
            }
        }
        messages
    }
}
