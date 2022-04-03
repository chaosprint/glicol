use std::{error, fmt};

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Init,
    Start,
    Stop,
    Cleanup,
    Task,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Error: {:?}.", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Init => "Bela_initAudio error",
            Error::Start => "Bela_startAudio error",
            Error::Stop => "Bela_stopAudio error",
            Error::Cleanup => "Bela_cleanupAudio error",
            Error::Task => "Bela_scheduleAuxiliaryTask error",
        }
    }
}
