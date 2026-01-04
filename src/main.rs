use futures_util::StreamExt;
use kalosm_sound::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let model = Whisper::new().await?;
    let mic = MicInput::default();

    let stream = mic.stream();

    println!("I should be listening now");
    let mut text_stream: ChunkedTranscriptionTask<_> = stream.transcribe(model);

    text_stream.next().await.unwrap().text();

    Ok(())
}
