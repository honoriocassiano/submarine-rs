use sdl2::{AudioSubsystem, EventPump, EventSubsystem, Sdl, VideoSubsystem};

#[allow(dead_code)]
pub struct SdlSystem {
    sdl_context: Sdl,

    audio: AudioSubsystem,
    video: VideoSubsystem,
    event: EventSubsystem,
    event_pump: EventPump,
}

#[allow(dead_code)]
impl SdlSystem {
    pub fn init() -> Result<SdlSystem, String> {
        let sdl_context = sdl2::init()?;

        let audio = sdl_context.audio()?;
        let video = sdl_context.video()?;
        let event = sdl_context.event()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            sdl_context,
            audio,
            video,
            event,
            event_pump,
        })
    }

    pub fn audio(&mut self) -> &mut AudioSubsystem {
        &mut self.audio
    }

    pub fn video(&mut self) -> &mut VideoSubsystem {
        &mut self.video
    }

    pub fn event(&mut self) -> &mut EventSubsystem {
        &mut self.event
    }

    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
