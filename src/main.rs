use futures_util::StreamExt;
use kalosm_sound::{rodio::Source, *};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mic = MicInput::default();

    let stream = mic.stream();

    let vad = stream.voice_activity_stream();
    let mut audio_chunks = vad.rechunk_voice_activity();

    while let Some(input) = audio_chunks.next().await {
        println!(
            "New voice activity chunk wityh duration {:?}",
            input.total_duration()
        );
        let mut ts = input.transcribe(Whisper::new().await?);

        while let Some(text) = ts.next().await {
            println!("{}", text.text());
        }
    }

    Ok(())
}
