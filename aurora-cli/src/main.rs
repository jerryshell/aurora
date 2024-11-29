fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let video_glob = std::env::var("VIDEO_GLOB").expect("VICEO_GLOB must be set");
    tracing::info!("video_glob: {video_glob}");

    let path_list = glob::glob(&video_glob)
        .expect("failed to read glob pattern")
        .filter_map(|e| e.ok())
        .map(|path| path.to_str().unwrap_or("").to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    tracing::info!("path_list: {path_list:?}");

    for video_filename in path_list {
        tracing::info!("video_filename: {video_filename}");

        let target_frame_rate = std::env::var("TARGET_FRAME_RATE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(60);
        tracing::info!("target_frame_rate: {target_frame_rate}");

        aurora_cli::run(&video_filename, target_frame_rate);
    }
}
