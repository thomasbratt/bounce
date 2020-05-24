use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkError {
    IntegerOverflows(String, u32),
    Height(),
    Title(),
    Sdl(String),
    Width(),
}

impl fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FrameworkError::*;
        match self {
            IntegerOverflows(name, value) => write!(f, "Integer overflows {} {}", name, value),
            Height() => write!(f, "Invalid window height"),
            Title() => write!(f, "Invalid window title"),
            Sdl(ref text) => write!(f, "SDL error: {}", text),
            Width() => write!(f, "Invalid window width"),
        }
    }
}

impl From<WindowBuildError> for FrameworkError {
    fn from(error: WindowBuildError) -> Self {
        match error {
            WindowBuildError::HeightOverflows(_h) => FrameworkError::Height(),
            WindowBuildError::WidthOverflows(_w) => FrameworkError::Width(),
            WindowBuildError::InvalidTitle(_ne) => FrameworkError::Title(),
            _ => FrameworkError::Sdl(String::from("Unknown WindowBuildError")),
        }
    }
}

impl From<IntegerOrSdlError> for FrameworkError {
    fn from(error: IntegerOrSdlError) -> Self {
        match error {
            IntegerOrSdlError::IntegerOverflows(name, value) => {
                FrameworkError::IntegerOverflows(String::from(name), value)
            }
            IntegerOrSdlError::SdlError(text) => FrameworkError::Sdl(text),
        }
    }
}
