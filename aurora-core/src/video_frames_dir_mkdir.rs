pub fn video_frames_dir_mkdir(video_filename: &str) -> String {
    let video_frames_dir_name = format!("{video_filename}_frames");
    tracing::info!("video_frames_dir_name {video_frames_dir_name}");

    let video_frames_dir_mkdir_result = std::fs::create_dir(&video_frames_dir_name);
    tracing::info!("video_frames_dir_mkdir_result: {video_frames_dir_mkdir_result:?}");

    video_frames_dir_name
}
