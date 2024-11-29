pub fn extract_audio(video_filename: &str) -> String {
    let video_extract_audio_filename = format!("{video_filename}_audio.m4a");
    tracing::info!("video_extract_audio_filename: {video_extract_audio_filename}");

    let extract_audio_cmd_str = if cfg!(windows) {
        format!(
            r"ffmpeg\ffmpeg.exe -y -i {video_filename} -vn -acodec copy {video_extract_audio_filename}"
        )
    } else {
        format!(
            r"ffmpeg/ffmpeg -y -i {video_filename} -vn -acodec copy {video_extract_audio_filename}"
        )
    };
    tracing::info!("extract_audio_cmd_str: {extract_audio_cmd_str}");

    let extract_audio_output = crate::execute_cmd(&extract_audio_cmd_str);
    tracing::info!("extract_audio_output: {extract_audio_output:?}");

    video_extract_audio_filename
}
