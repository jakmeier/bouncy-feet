use clap::{Arg, Command};

fn main() -> anyhow::Result<()> {
    let mut cmd = Command::new("Bouncy Lab")
        .subcommand(plot_body_movements())
        .subcommand(video_detection());

    let matches = cmd.clone().get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("plot-body-movements") {
        let file_path: &String = sub_matches.get_one("input").expect("input file required");
        return bouncy_lab::plot_body_part_time_series(file_path);
    }

    if let Some(sub_matches) = matches.subcommand_matches("video-detect") {
        let file_path: &String = sub_matches.get_one("input").expect("input file required");
        return bouncy_lab::video_pose_detection(file_path);
    }

    cmd.print_help()?;
    Ok(())
}

fn plot_body_movements() -> Command {
    Command::new("plot-body-movements")
        .about("Takes an input RON and plots how each body parts moved over time.")
        .arg(Arg::new("input").required(true))
}

fn video_detection() -> Command {
    Command::new("video-detect")
        .about("Run computer vision tasks to detect a person and their pose in a video.")
        .arg(Arg::new("input").required(true))
}
