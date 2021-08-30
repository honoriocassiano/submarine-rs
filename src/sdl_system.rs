use sdl2::{AudioSubsystem, EventSubsystem, Sdl, VideoSubsystem};

#[allow(dead_code)]
pub struct SdlSystem {
    sdl_context: Sdl,

    audio: AudioSubsystem,
    video: VideoSubsystem,
    event: EventSubsystem,
}

#[allow(dead_code)]
impl SdlSystem {
    pub fn init() -> Result<SdlSystem, String> {
        let sdl_context = sdl2::init()?;

        let audio = sdl_context.audio()?;
        let video = sdl_context.video()?;
        let event = sdl_context.event()?;

        Ok(Self {
            sdl_context,
            audio,
            video,
            event,
        })
    }

    pub fn sdl_context(&self) -> &Sdl {
        &self.sdl_context
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
}
