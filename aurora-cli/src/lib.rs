fn execute_cmd(cmd: &str) -> std::process::Output {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .expect("failed to execute process")
    }
}

fn extract_audio(video_filename: &str) -> String {
    let video_extract_audio_filename = format!("{video_filename}_audio.m4a");
    println!("video_extract_audio_filename: {video_extract_audio_filename}");

    let extract_audio_cmd_str =
        format!("ffmpeg -y -i {video_filename} -vn -acodec copy {video_extract_audio_filename}");
    println!("extract_audio_cmd_str: {extract_audio_cmd_str}");

    let extract_audio_output = execute_cmd(&extract_audio_cmd_str);
    println!("extract_audio_output: {extract_audio_output:?}");
    video_extract_audio_filename
}

fn video_frames_dir_mkdir(video_filename: &str) -> String {
    let video_frames_dir_name = format!("{video_filename}_frames");
    println!("video_frames_dir_name {video_frames_dir_name}");

    let video_frames_dir_mkdir_cmd_str = format!("mkdir {video_frames_dir_name}");
    println!("video_frames_dir_mkdir_cmd_str: {video_frames_dir_mkdir_cmd_str}");

    let mkdir_cmd_output = execute_cmd(&video_frames_dir_mkdir_cmd_str);
    println!("mkdir_cmd_output: {mkdir_cmd_output:?}");

    video_frames_dir_name
}

fn decode_frames(video_filename: &str, video_frames_dir_name: &str) {
    let decode_frames_cmd =
        format!("ffmpeg -y -i {video_filename} {video_frames_dir_name}/frame_%08d.png");
    println!("decode_frames_cmd: {decode_frames_cmd}");

    let decode_frames_cmd_output = execute_cmd(&decode_frames_cmd);
    println!("decode_frames_cmd_output: {decode_frames_cmd_output:?}");
}

fn get_origin_frame_count(video_frames_dir_name: &str) -> usize {
    let origin_frame_count = std::fs::read_dir(video_frames_dir_name).unwrap().count();
    println!("origin_frame_count: {origin_frame_count}");

    origin_frame_count
}

fn get_origin_frame_rate(video_filename: &str) -> f32 {
    let ffprobe_cmd_str = format!("ffprobe {video_filename}");
    println!("ffprobe_cmd_str: {ffprobe_cmd_str}");

    let ffprobe_cmd_output = execute_cmd(&ffprobe_cmd_str);
    println!("ffprobe_cmd_output: {ffprobe_cmd_output:?}");

    let origin_frame_rate_regex = regex::Regex::new(r"([0-9]+(\.?[0-9]+)) fps").unwrap();
    let ffprobe_cmd_output_str = String::from_utf8(ffprobe_cmd_output.stderr).unwrap();
    println!("ffprobe_cmd_output_str: {ffprobe_cmd_output_str}");

    let re_cap = origin_frame_rate_regex
        .captures(&ffprobe_cmd_output_str)
        .unwrap();
    println!("re_cap: {re_cap:?}");

    let origin_frame_rate_str = re_cap.iter().nth(1).unwrap().unwrap().as_str();
    println!("origin_frame_rate_str: {origin_frame_rate_str}");

    let origin_frame_rate = origin_frame_rate_str.parse::<f32>().unwrap();
    println!("origin_frame_rate: {origin_frame_rate}");
    origin_frame_rate
}

fn video_interpolate_frames_dir_mkdir(video_filename: &str) -> String {
    let video_interpolate_frames_dir_name = format!("{video_filename}_interpolate_frames");
    println!("video_interpolate_frames_dir_name: {video_interpolate_frames_dir_name}");

    let mkdir_cmd_str = format!("mkdir {video_interpolate_frames_dir_name}");
    println!("mkdir_cmd_str: {mkdir_cmd_str}");

    let mkdir_cmd_output = execute_cmd(&mkdir_cmd_str);
    println!("mkdir_cmd_output: {mkdir_cmd_output:?}");
    video_interpolate_frames_dir_name
}

fn interpolate_frame(
    target_frame_count: usize,
    video_frames_dir_name: &str,
    video_interpolate_frames_dir_name: &str,
) {
    let interpolate_frame_cmd_str = format!("rife-ncnn-vulkan/rife-ncnn-vulkan -m rife-v4.6 -n {target_frame_count} -i {video_frames_dir_name} -o {video_interpolate_frames_dir_name}");
    println!("interpolate_frame_cmd_str: {interpolate_frame_cmd_str}");

    let interpolate_frame_cmd_output = execute_cmd(&interpolate_frame_cmd_str);
    println!("interpolate_frame_cmd_output: {interpolate_frame_cmd_output:?}");
}

fn encode_video(
    target_frame_rate: f32,
    video_interpolate_frames_dir_name: &str,
    video_filename: &str,
) {
    let encode_video_cmd_str = format!("ffmpeg -y -framerate {target_frame_rate} -i {video_interpolate_frames_dir_name}/%08d.png -i {video_filename}_audio.m4a -c:a copy -crf 20 -c:v libx264 -pix_fmt yuv420p output_{video_filename}.mp4");
    println!("encode_video_cmd_str: {encode_video_cmd_str}");

    let encode_video_cmd_output = execute_cmd(&encode_video_cmd_str);
    println!("encode_video_cmd_output: {encode_video_cmd_output:?}");
}

pub fn clean(
    video_extract_audio_filename: &str,
    video_frames_dir_name: &str,
    video_interpolate_frames_dir_name: &str,
) {
    let remove_video_extract_audio_result = std::fs::remove_file(video_extract_audio_filename);
    println!("remove_video_extract_audio_result: {remove_video_extract_audio_result:?}");

    let remove_video_frames_dir_result = std::fs::remove_dir_all(video_frames_dir_name);
    println!("remove_video_frames_dir_result: {remove_video_frames_dir_result:?}");

    let remove_video_interpolate_frames_dir_result =
        std::fs::remove_dir_all(video_interpolate_frames_dir_name);
    println!(
        "remove_video_interpolate_frames_dir_result: {remove_video_interpolate_frames_dir_result:?}"
    );
}

pub fn run(video_filename: &str, frame_multiple: usize) {
    let video_extract_audio_filename = extract_audio(video_filename);

    let video_frames_dir_name = video_frames_dir_mkdir(video_filename);

    decode_frames(video_filename, &video_frames_dir_name);

    let origin_frame_count = get_origin_frame_count(&video_frames_dir_name);

    let origin_frame_rate = get_origin_frame_rate(video_filename);

    let target_frame_count = frame_multiple * origin_frame_count;
    println!("target_frame_count: {target_frame_count}");

    let video_interpolate_frames_dir_name = video_interpolate_frames_dir_mkdir(video_filename);

    interpolate_frame(
        target_frame_count,
        &video_frames_dir_name,
        &video_interpolate_frames_dir_name,
    );

    let target_frame_rate = frame_multiple as f32 * origin_frame_rate;
    println!("target_frame_rate: {target_frame_rate}");

    encode_video(
        target_frame_rate,
        &video_interpolate_frames_dir_name,
        video_filename,
    );

    clean(
        &video_extract_audio_filename,
        &video_frames_dir_name,
        &video_interpolate_frames_dir_name,
    );
}
