use std::fs;
use std::io;
use std::io::Write as _;
use std::process;

fn main() -> io::Result<()> {
    let tokens = windows_macros::generate!(
        Windows::Win32::CoreAudio::*,
        Windows::Win32::Audio::*,
        Windows::Win32::Multimedia::{
            WAVEFORMATEX,
            WAVEFORMATEXTENSIBLE,
            WAVE_FORMAT_PCM,
            WAVE_FORMAT_IEEE_FLOAT,
            KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
        },
        Windows::Win32::StructuredStorage::PROPVARIANT,
        Windows::Win32::Com::{CoTaskMemAlloc, CoTaskMemFree, CLSIDFromProgID, CoInitializeEx, CoCreateInstance, CLSCTX},
        Windows::Win32::WindowsAndMessaging::GetForegroundWindow,
        Windows::Win32::SystemServices::{CreateEventA, ResetEvent, WaitForSingleObject, WAIT_RETURN_CAUSE, HANDLE},
        Windows::Win32::WindowsProgramming::INFINITE,
    );

    let path = windows_gen::workspace_dir()
        .join("rotary-device")
        .join("src")
        .join("bindings.rs");

    let mut file = fs::File::create(&path)?;
    file.write_all(
        "// This file was generated by the `windows` crate - do not edit by hand!\n\n".as_bytes(),
    )?;
    file.write_all(tokens.as_bytes())?;
    drop(file);

    let mut cmd = process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
