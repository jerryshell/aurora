pub fn extract_audio(video_filename: &str) -> String {
    let video_extract_audio_filename = format!("{video_filename}_audio.m4a");
    println!("video_extract_audio_filename: {video_extract_audio_filename}");

    let extract_audio_cmd_str = if cfg!(target_os = "windows") {
        format!(
            r"ffmpeg\ffmpeg.exe -y -i {video_filename} -vn -acodec copy {video_extract_audio_filename}"
        )
    } else {
        format!(
            r"ffmpeg/ffmpeg -y -i {video_filename} -vn -acodec copy {video_extract_audio_filename}"
        )
    };
    println!("extract_audio_cmd_str: {extract_audio_cmd_str}");

    let extract_audio_output = crate::execute_cmd(&extract_audio_cmd_str);
    println!("extract_audio_output: {extract_audio_output:?}");
    video_extract_audio_filename
}
