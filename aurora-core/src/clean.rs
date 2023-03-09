pub fn clean(
    video_extract_audio_filename: &str,
    video_frames_dir_name: &str,
    video_interpolate_frames_dir_name: &str,
) {
    if !video_extract_audio_filename.is_empty() {
        let remove_video_extract_audio_result = std::fs::remove_file(video_extract_audio_filename);
        tracing::info!("remove_video_extract_audio_result: {remove_video_extract_audio_result:?}");
    }

    if !video_frames_dir_name.is_empty() {
        let remove_video_frames_dir_result = std::fs::remove_dir_all(video_frames_dir_name);
        tracing::info!("remove_video_frames_dir_result: {remove_video_frames_dir_result:?}");
    }

    if !video_interpolate_frames_dir_name.is_empty() {
        let remove_video_interpolate_frames_dir_result =
            std::fs::remove_dir_all(video_interpolate_frames_dir_name);
        tracing::info!("remove_video_interpolate_frames_dir_result: {remove_video_interpolate_frames_dir_result:?}");
    }
}
