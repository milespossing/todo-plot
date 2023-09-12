use chrono::{DateTime, Local, NaiveDateTime};
use clap::Parser;
use rgb::RGB8;
use std::str::FromStr;
use textplots::{AxisBuilder, Chart, ColorPlot, LabelBuilder, LabelFormat, LineStyle, Shape};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct ReportLine {
    date: DateTime<Local>,
    open: u8,
    closed: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct ReportLineParsingError;

impl FromStr for ReportLine {
    type Err = ReportLineParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<&str>>();
        let naive = split[0]
            .parse::<NaiveDateTime>()
            .expect("Could not parse date");
        let date = naive.and_local_timezone(Local).single().unwrap();
        let open: u8 = split[1].parse::<u8>().expect("Could not open tasks");
        let closed: u8 = split[2].parse::<u8>().expect("Could not open tasks");
        Ok(ReportLine { date, open, closed })
    }
}

fn to_point(min_date: &DateTime<Local>, date: &DateTime<Local>, count: &u8) -> (f32, f32) {
    let diff = *date - *min_date;
    let minutes = diff.num_seconds() as f32;
    let x: f32 = minutes;
    let y: f32 = *count as f32;
    (x, y)
}

fn get_points_from_report_lines<'a>(lines: Vec<ReportLine>) -> (Vec<(f32, f32)>, Vec<(f32, f32)>) {
    let min = lines[0].date;
    lines
        .into_iter()
        .map(|line| {
            (
                to_point(&min, &line.date, &(line.closed)),
                to_point(&min, &line.date, &(line.open + line.closed)),
            )
        })
        .unzip()
}

fn main() {
    let args = Cli::parse();
    let entries = std::fs::read_to_string(args.path)
        .unwrap()
        .lines()
        .map(|l| ReportLine::from_str(l).unwrap())
        .collect::<Vec<ReportLine>>();
    let (completed, open) = get_points_from_report_lines(entries);
    let max: f32 = completed.last().copied().unwrap().0;
    let red = RGB8::new(255u8, 0u8, 0u8);
    let green = RGB8::new(0u8, 255u8, 0u8);

    Chart::new(180, 60, -10f32, max + 10f32)
        .linecolorplot(&Shape::Lines(&completed), green)
        .linecolorplot(&Shape::Lines(&open), red)
        .x_axis_style(LineStyle::None)
        .y_axis_style(LineStyle::None)
        .x_label_format(LabelFormat::None)
        .y_label_format(LabelFormat::None)
        .display();
}
