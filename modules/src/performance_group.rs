use crate::sound::sound;

pub mod performance_group {
    pub use crate::sound::sound::instrument;

    pub fn music_group() {
        instrument::woodwind::clarinet();
        instrument::strings::guitar();
    }
}

