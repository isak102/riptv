mod get_url;

use crate::StreamType;
use std::path::PathBuf;
use std::{error::Error, result::Result};

pub fn play(
    stream_type: StreamType,
    data_directory: &PathBuf,
    _fzf: bool,
) -> Result<(), Box<dyn Error>> {
    let url = get_url::dmenu(stream_type, data_directory)?;

    Ok(())
}
