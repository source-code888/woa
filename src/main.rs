#![allow(unused)]
mod whale;
mod woa;
mod jssp_instance;

extern crate nalgebra as na;

use plotters::prelude::*;

use na::DVector;
use std::error::Error;
use std::f64::consts::PI;
use crate::jssp_instance::{Instance, JSSPInstance};


fn objective_function(position: &DVector<f64>) -> f64 {
    let mut f: f64 = 0f64;
    for i in 0..position.len() {
        f += position[i].powf(2f64) - 10f64 * (2f64 * PI * position[i]).cos() + 10f64
    }
    f
}

fn f_8(position: &DVector<f64>) -> f64 {
    let mut f: f64 = 0f64;
    for i in 0..position.len() {
        f = -position[i] * (position[i].abs().sqrt()).sin();
    }
    f
}


fn create_graphic_2d(
    filename: &str,
    caption: &str,
    x_range: (usize, usize),
    y_range: (f64, f64),
    plots: &[f64]
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(&filename, (640, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(caption, ("sans-serif", 20))
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (0..plots.len()).map(|t| (t, plots[t])),
        &BLUE,
    ))?;
    open::that(&filename)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let instance: Instance = Instance::LA05;
    let instance = JSSPInstance::from_instance(instance, String::from("src/jssp_instance/lit/"))?;
    println!("Sequences: ");
    instance.sequences.iter()
        .for_each(
            |i| println!("{i:?}")
        );
    println!("Processing times: ");
    instance.processing_times.iter()
        .for_each(|p| println!("{p:?}"));
    Ok(())
}
