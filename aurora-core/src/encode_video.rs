pub fn encode_video(
    target_frame_rate: f32,
    video_interpolate_frames_dir_name: &str,
    video_filename: &str,
) {
    let video_encoder = std::env::var("VIDEO_ENCODER").unwrap_or("libx264".to_owned());
    tracing::info!("video_encoder: {video_encoder}");

    let encode_video_cmd_str = if cfg!(windows) {
        format!(
            r"ffmpeg\ffmpeg.exe -y -framerate {target_frame_rate} -i {video_interpolate_frames_dir_name}/%08d.png -i {video_filename}_audio.m4a -c:a copy -crf 20 -c:v {video_encoder} -pix_fmt yuv420p output_{video_filename}.mp4"
        )
    } else {
        format!(
            r"ffmpeg/ffmpeg -y -framerate {target_frame_rate} -i {video_interpolate_frames_dir_name}/%08d.png -i {video_filename}_audio.m4a -c:a copy -crf 20 -c:v {video_encoder} -pix_fmt yuv420p output_{video_filename}.mp4"
        )
    };
    tracing::info!("encode_video_cmd_str: {encode_video_cmd_str}");

    let encode_video_cmd_output = crate::execute_cmd(&encode_video_cmd_str);
    tracing::info!("encode_video_cmd_output: {encode_video_cmd_output:?}");
}
