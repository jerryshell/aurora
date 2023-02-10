fn main() {
    dotenv::dotenv().ok();

    let video_filename = std::env::var("VIDEO_FILENAME").expect("VIDEO_FILENAME must be set");
    println!("video_filename: {video_filename}");

    let frame_multiple = match std::env::var("FRAME_MULTIPLE") {
        Ok(str) => str.parse::<usize>().unwrap_or(2),
        Err(_) => 2,
    };
    println!("frame_multiple: {frame_multiple}");

    aurora_cli::run(&video_filename, frame_multiple);
}
