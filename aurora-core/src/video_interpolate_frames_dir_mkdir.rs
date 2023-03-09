pub fn video_interpolate_frames_dir_mkdir(video_filename: &str) -> String {
    let video_interpolate_frames_dir_name = format!("{video_filename}_interpolate_frames");
    println!("video_interpolate_frames_dir_name: {video_interpolate_frames_dir_name}");

    let video_interpolate_frames_dir_mkdir_result =
        std::fs::create_dir(&video_interpolate_frames_dir_name);
    println!(
        "video_interpolate_frames_dir_mkdir_result: {video_interpolate_frames_dir_mkdir_result:?}"
    );

    video_interpolate_frames_dir_name
}
