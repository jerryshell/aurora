pub fn get_origin_frame_count(video_frames_dir_name: &str) -> usize {
    let origin_frame_count = std::fs::read_dir(video_frames_dir_name).unwrap().count();
    tracing::info!("origin_frame_count: {origin_frame_count}");

    origin_frame_count
}
