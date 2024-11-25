use anyhow::{anyhow, Error};
use chrono::prelude::*;
use plotters::prelude::*;

pub struct PlotArgs {
    pub image_path: String,
    pub image_height: u32,
    pub image_width: u32,
    pub caption: String,
    pub caption_font: String,
    pub caption_font_size: u32,
    pub left_label_area_size: u32,
    pub right_label_area_size: u32,
    pub bottom_label_area_size: u32,
    pub x_label_area_size: u32,
    pub y_label_area_size: u32,
    pub x_description: String,
    pub y_description: String,
}
impl Default for PlotArgs {
    fn default() -> Self {
        Self {
            image_path: "temp_chart.png".into(),
            image_height: 800,
            image_width: 800,
            caption: "Temperature vs Time".into(),
            caption_font: "Arial".into(),
            caption_font_size: 40,
            left_label_area_size: 40,
            right_label_area_size: 40,
            bottom_label_area_size: 40,
            x_label_area_size: 50,
            y_label_area_size: 50,
            x_description: "DateTime UTC".into(),
            y_description: "Temperature °F".into(),
        }
    }
}

pub fn plot(data_set: Vec<(DateTime<Utc>, f64)>, args: PlotArgs) -> Result<(), Error> {
    let PlotArgs {
        image_path,
        image_height,
        image_width,
        caption,
        caption_font,
        caption_font_size,
        left_label_area_size,
        right_label_area_size,
        bottom_label_area_size,
        x_label_area_size,
        y_label_area_size,
        x_description,
        y_description,
    } = args;

    let (date_min, date_max, temp_min, temp_max) = find_bounds(&data_set)?;

    let drawing_area =
        BitMapBackend::new(&image_path, (image_width, image_height)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(caption, (caption_font.as_str(), caption_font_size))
        .set_label_area_size(LabelAreaPosition::Left, left_label_area_size)
        .set_label_area_size(LabelAreaPosition::Bottom, bottom_label_area_size)
        .set_label_area_size(LabelAreaPosition::Right, right_label_area_size)
        .x_label_area_size(x_label_area_size)
        .y_label_area_size(y_label_area_size)
        .build_cartesian_2d(date_min..date_max, temp_min..temp_max)?;

    chart
        .configure_mesh()
        .x_desc(x_description)
        .y_desc(y_description)
        .x_label_formatter(&|x| x.date_naive().to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(data_set.into_iter(), &RED))?;

    Ok(())
}

fn find_bounds(
    data_set: &[(DateTime<Utc>, f64)],
) -> Result<(DateTime<Utc>, DateTime<Utc>, f64, f64), Error> {
    #[rustfmt::skip]
    let (date_min, date_max, temp_min, temp_max) = (
        data_set.iter().min_by_key(|(d, _)| d).ok_or(anyhow!("no date minimum. is data_set empty?"))?.0,
        data_set.iter().max_by_key(|(d, _)| d).ok_or(anyhow!("no date maximum. is data_set empty?"))?.0,
        data_set.iter().min_by(|(_, t1), (_, t2)| t1.total_cmp(t2)).ok_or(anyhow!("no temp minimum. is data_set empty?"))?.1,
        data_set.iter().max_by(|(_, t1), (_, t2)| t1.total_cmp(t2)).ok_or(anyhow!("no temp maximum. is data_set empty?"))?.1,
    );
    Ok((date_min, date_max, temp_min, temp_max))
}
