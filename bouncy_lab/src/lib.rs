use keypoints::Keypoints;
use plotters::prelude::*;
use std::fs::File;
use std::io::Read;

mod keypoints;

pub fn plot_body_part_time_series(file_path: &str) -> anyhow::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Vec<(u64, Keypoints)> = ron::from_str(&contents)?;

    let root = BitMapBackend::new("output.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_t = data.first().expect("no data found").0;
    let max_t = data.last().expect("no data found").0;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Time Series of Body Coordinates",
            ("sans-serif", 50).into_font(),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(min_t..max_t, 0f32..1f32)?;

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Coordinate")
        .draw()?;

    let left_heel = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.heel);
    let left_ankle = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.ankle);
    let left_toes = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.toes);
    let left_knee = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.knee);
    let left_hip = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.hip);
    let left_shoulder = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.left.shoulder);
    let right_heel = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.heel);
    let right_ankle = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.ankle);
    let right_toes = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.toes);
    let right_knee = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.knee);
    let right_hip = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.hip);
    let right_shoulder = |(t, keypoints): &(u64, Keypoints)| (*t, keypoints.right.shoulder);

    draw_body_part(&data, &left_heel, &mut chart, "Left Heel", 1.0)?;
    draw_body_part(&data, &right_heel, &mut chart, "Right Heel", 0.5)?;

    draw_body_part(&data, &left_toes, &mut chart, "Left Toes", 1.0)?;
    draw_body_part(&data, &right_toes, &mut chart, "Right Toes", 0.5)?;

    draw_body_part(&data, &left_ankle, &mut chart, "Left Ankle", 1.0)?;
    draw_body_part(&data, &right_ankle, &mut chart, "Right Ankle", 0.5)?;

    draw_body_part(&data, &left_knee, &mut chart, "Left Knee", 1.0)?;
    draw_body_part(&data, &right_knee, &mut chart, "Right Knee", 0.5)?;

    draw_body_part(&data, &right_hip, &mut chart, "Right Hip", 0.5)?;
    draw_body_part(&data, &left_hip, &mut chart, "Left Hip", 1.0)?;

    draw_body_part(&data, &left_shoulder, &mut chart, "Left Shoulder", 1.0)?;
    draw_body_part(&data, &right_shoulder, &mut chart, "Right Shoulder", 0.5)?;

    // Draw the legend
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

fn draw_body_part(
    timestamp_keypoint_tuples: &[(u64, Keypoints)],
    left_heel: &impl Fn(&(u64, Keypoints)) -> (u64, keypoints::Cartesian3d),
    chart: &mut ChartContext<
        BitMapBackend,
        Cartesian2d<plotters::coord::types::RangedCoordu64, plotters::coord::types::RangedCoordf32>,
    >,
    body_part: &str,
    shade: f32,
) -> anyhow::Result<()> {
    let bright = (255.0 * shade) as u8;
    let dark = 0;
    let red = RGBAColor(bright, dark, dark, 1.0);
    let green = RGBAColor(dark, bright, dark, 1.0);
    let blue = RGBAColor(dark, dark, bright, 1.0);

    let x: Vec<(u64, f32)> = timestamp_keypoint_tuples
        .iter()
        .map(left_heel)
        .map(|(t, coordinate)| (t, coordinate.x))
        .collect();
    chart
        .draw_series(LineSeries::new(x, &red))?
        .label(format!("{}-x", body_part))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], red));
    let y: Vec<(u64, f32)> = timestamp_keypoint_tuples
        .iter()
        .map(left_heel)
        .map(|(t, coordinate)| (t, coordinate.y))
        .collect();
    chart
        .draw_series(LineSeries::new(y, &green))?
        .label(format!("{}-y", body_part))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], green));
    let z: Vec<(u64, f32)> = timestamp_keypoint_tuples
        .iter()
        .map(left_heel)
        .map(|(t, coordinate)| (t, coordinate.z))
        .collect();
    chart
        .draw_series(LineSeries::new(z, &blue))?
        .label(format!("{}-z", body_part))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], blue));
    Ok(())
}
