pub fn get_origin_frame_rate(video_filename: &str) -> f32 {
    let ffprobe_cmd_str = if cfg!(windows) {
        format!(r"ffmpeg\ffprobe.exe {video_filename}")
    } else {
        format!(r"ffmpeg/ffprobe {video_filename}")
    };
    tracing::info!("ffprobe_cmd_str: {ffprobe_cmd_str}");

    let ffprobe_cmd_output = crate::execute_cmd(&ffprobe_cmd_str);
    tracing::info!("ffprobe_cmd_output: {ffprobe_cmd_output:?}");

    let origin_frame_rate_regex = regex::Regex::new(r"([0-9]+(\.?[0-9]+)) fps").unwrap();
    let ffprobe_cmd_output_str = String::from_utf8(ffprobe_cmd_output.stderr).unwrap();
    tracing::info!("ffprobe_cmd_output_str: {ffprobe_cmd_output_str}");

    let re_cap = origin_frame_rate_regex
        .captures(&ffprobe_cmd_output_str)
        .unwrap();
    tracing::info!("re_cap: {re_cap:?}");

    let origin_frame_rate_str = re_cap.iter().nth(1).unwrap().unwrap().as_str();
    tracing::info!("origin_frame_rate_str: {origin_frame_rate_str}");

    let origin_frame_rate = origin_frame_rate_str.parse::<f32>().unwrap();
    tracing::info!("origin_frame_rate: {origin_frame_rate}");

    origin_frame_rate
}
