pub fn decode_frames(video_filename: &str, video_frames_dir_name: &str) {
    let decode_frames_cmd_str = if cfg!(target_os = "windows") {
        format!(
            r"ffmpeg\ffmpeg.exe -y -vsync passthrough -i {video_filename} {video_frames_dir_name}/frame_%08d.png"
        )
    } else {
        format!(
            r"ffmpeg/ffmpeg -y -vsync passthrough -i {video_filename} {video_frames_dir_name}/frame_%08d.png"
        )
    };
    println!("decode_frames_cmd_str: {decode_frames_cmd_str}");

    let decode_frames_cmd_output = crate::execute_cmd(&decode_frames_cmd_str);
    println!("decode_frames_cmd_output: {decode_frames_cmd_output:?}");
}
