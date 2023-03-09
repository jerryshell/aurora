fn get_j() -> String {
    match std::env::var("J") {
        Ok(str) => str,
        Err(_) => {
            let cpus = num_cpus::get();
            format!("{}:{}:{}", (cpus - 1).max(1), (cpus - 1).max(2), 2)
        }
    }
}

pub fn run(video_filename: &str, target_frame_rate: usize) {
    let origin_frame_rate = aurora_core::get_origin_frame_rate(video_filename);
    println!("origin_frame_rate: {origin_frame_rate}");

    let frame_multiple = (target_frame_rate as f32 / origin_frame_rate).ceil() as usize;
    println!("frame_multiple: {frame_multiple}");
    if frame_multiple <= 1 {
        println!("frame_multiple <= 1, skip");
        return;
    }

    let video_extract_audio_filename = aurora_core::extract_audio(video_filename);
    println!("video_extract_audio_filename: {video_extract_audio_filename}");

    let video_frames_dir_name = aurora_core::video_frames_dir_mkdir(video_filename);
    println!("video_frames_dir_name: {video_frames_dir_name}");

    aurora_core::decode_frames(video_filename, &video_frames_dir_name);

    let origin_frame_count = aurora_core::get_origin_frame_count(&video_frames_dir_name);
    println!("origin_frame_count: {origin_frame_count}");

    let target_frame_count = frame_multiple * origin_frame_count;
    println!("target_frame_count: {target_frame_count}");

    let video_interpolate_frames_dir_name =
        aurora_core::video_interpolate_frames_dir_mkdir(video_filename);
    println!("video_interpolate_frames_dir_name: {video_interpolate_frames_dir_name}");

    let j = get_j();
    println!("j: {j}");

    aurora_core::interpolate_frame(
        target_frame_count,
        &video_frames_dir_name,
        &video_interpolate_frames_dir_name,
        &j,
    );

    let target_frame_rate = frame_multiple as f32 * origin_frame_rate;
    println!("target_frame_rate: {target_frame_rate}");

    aurora_core::encode_video(
        target_frame_rate,
        &video_interpolate_frames_dir_name,
        video_filename,
    );

    aurora_core::clean(
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
