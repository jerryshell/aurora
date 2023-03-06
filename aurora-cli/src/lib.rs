fn execute_cmd(cmd: &str) -> std::process::Output {
    if cfg!(target_os = "windows") {
        println!("windows execute_cmd: {cmd}");
        let output = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process");
        println!("windows exe ok");
        output
    } else {
        println!("unix execute_cmd: {cmd}");
        let output = std::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .expect("failed to execute process");
        println!("unix exe ok");
        output
    }
}

fn extract_audio(video_filename: &str) -> String {
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

    let extract_audio_output = execute_cmd(&extract_audio_cmd_str);
    println!("extract_audio_output: {extract_audio_output:?}");
    video_extract_audio_filename
}

fn video_frames_dir_mkdir(video_filename: &str) -> String {
    let video_frames_dir_name = format!("{video_filename}_frames");
    println!("video_frames_dir_name {video_frames_dir_name}");

    let video_frames_dir_mkdir_result = std::fs::create_dir(&video_frames_dir_name);
    println!("video_frames_dir_mkdir_result: {video_frames_dir_mkdir_result:?}");

    video_frames_dir_name
}

fn decode_frames(video_filename: &str, video_frames_dir_name: &str) {
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

    let decode_frames_cmd_output = execute_cmd(&decode_frames_cmd_str);
    println!("decode_frames_cmd_output: {decode_frames_cmd_output:?}");
}

fn get_origin_frame_count(video_frames_dir_name: &str) -> usize {
    let origin_frame_count = std::fs::read_dir(video_frames_dir_name).unwrap().count();
    println!("origin_frame_count: {origin_frame_count}");

    origin_frame_count
}

fn get_origin_frame_rate(video_filename: &str) -> f32 {
    let ffprobe_cmd_str = if cfg!(target_os = "windows") {
        format!(r"ffmpeg\ffprobe.exe {video_filename}")
    } else {
        format!(r"ffmpeg/ffprobe {video_filename}")
    };
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

    let video_interpolate_frames_dir_mkdir_result =
        std::fs::create_dir(&video_interpolate_frames_dir_name);
    println!(
        "video_interpolate_frames_dir_mkdir_result: {video_interpolate_frames_dir_mkdir_result:?}"
    );

    video_interpolate_frames_dir_name
}

fn get_j() -> String {
    match std::env::var("J") {
        Ok(str) => str,
        Err(_) => {
            let cpus = num_cpus::get();
            format!("{}:{}:{}", cpus, cpus, cpus)
        }
    }
}

fn interpolate_frame(
    target_frame_count: usize,
    video_frames_dir_name: &str,
    video_interpolate_frames_dir_name: &str,
) {
    let j = get_j();
    println!("j: {j}");

    let interpolate_frame_cmd_str = if cfg!(target_os = "windows") {
        format!(
            r"rife-ncnn-vulkan\rife-ncnn-vulkan.exe -m rife-v4.6 -j {j} -n {target_frame_count} -i {video_frames_dir_name} -o {video_interpolate_frames_dir_name}"
        )
    } else {
        format!(
            r"rife-ncnn-vulkan/rife-ncnn-vulkan -m rife-v4.6 -j {j} -n {target_frame_count} -i {video_frames_dir_name} -o {video_interpolate_frames_dir_name}"
        )
    };
    println!("interpolate_frame_cmd_str: {interpolate_frame_cmd_str}");

    let interpolate_frame_cmd_output = execute_cmd(&interpolate_frame_cmd_str);
    println!("interpolate_frame_cmd_output: {interpolate_frame_cmd_output:?}");
}

fn encode_video(
    target_frame_rate: f32,
    video_interpolate_frames_dir_name: &str,
    video_filename: &str,
) {
    let video_encoder = match std::env::var("VIDEO_ENCODER") {
        Ok(str) => str,
        Err(_) => "libx264".to_owned(),
    };
    println!("video_encoder: {video_encoder}");
    let encode_video_cmd_str = if cfg!(target_os = "windows") {
        format!(
            r"ffmpeg\ffmpeg.exe -y -framerate {target_frame_rate} -i {video_interpolate_frames_dir_name}/%08d.png -i {video_filename}_audio.m4a -c:a copy -crf 20 -c:v {video_encoder} -pix_fmt yuv420p output_{video_filename}.mp4"
        )
    } else {
        format!(
            r"ffmpeg/ffmpeg -y -framerate {target_frame_rate} -i {video_interpolate_frames_dir_name}/%08d.png -i {video_filename}_audio.m4a -c:a copy -crf 20 -c:v {video_encoder} -pix_fmt yuv420p output_{video_filename}.mp4"
        )
    };
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

pub fn run(video_filename: &str, target_frame_rate: usize) {
    let video_extract_audio_filename = extract_audio(video_filename);
    println!("video_extract_audio_filename: {video_extract_audio_filename}");

    let video_frames_dir_name = video_frames_dir_mkdir(video_filename);
    println!("video_frames_dir_name: {video_frames_dir_name}");

    decode_frames(video_filename, &video_frames_dir_name);

    let origin_frame_count = get_origin_frame_count(&video_frames_dir_name);
    println!("origin_frame_count: {origin_frame_count}");

    let origin_frame_rate = get_origin_frame_rate(video_filename);
    println!("origin_frame_rate: {origin_frame_rate}");

    let frame_multiple = (target_frame_rate as f32 / origin_frame_rate).ceil() as usize;
    println!("frame_multiple: {frame_multiple}");

    let target_frame_count = frame_multiple * origin_frame_count;
    println!("target_frame_count: {target_frame_count}");

    let video_interpolate_frames_dir_name = video_interpolate_frames_dir_mkdir(video_filename);
    println!("video_interpolate_frames_dir_name: {video_interpolate_frames_dir_name}");

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

#[cfg(test)]
mod tests {
    mod j {
        #[test]
        fn test() {
            dotenv::dotenv().ok();
            let j = std::env::var("J").unwrap();
            assert_eq!(j, "1:2:2");
        }
    }

    mod ceil {
        #[test]
        fn test() {
            assert_eq!(4.0f32.ceil(), 4.0);
            assert_eq!(4.1f32.ceil(), 5.0);
        }
    }
}
