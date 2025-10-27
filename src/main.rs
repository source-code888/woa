mod whale;
mod woa;
mod jssp_instance;

extern crate nalgebra as na;

use plotters::prelude::*;

use crate::whale::Whale;
use crate::woa::WOA;
use na::DVector;
use std::error::Error;
use std::f64::consts::PI;

#[allow(unused)]
fn objective_function(position: &DVector<f64>) -> f64 {
    let mut f: f64 = 0f64;
    for i in 0..position.len() {
        f += position[i].powf(2f64) - 10f64 * (2f64 * PI * position[i]).cos() + 10f64
    }
    f
}

#[allow(unused)]
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

fn main() {
    let population_size: usize = 40;
    let dim = 3;
    //Lower, Upper
    let bounds: (f64, f64) = (-5.12, 5.12);
    let maximization: bool = false;
    let max_iterations: usize = 500;
    let mut algorithm: WOA = WOA::initialize(
        dim,
        population_size,
        bounds,
        maximization,
        max_iterations,
        objective_function,
    );
    let history = algorithm.run();
    let whale = Whale::from_fn(|| history.last().unwrap().clone());
    let all_fitness: Vec<f64> = history.iter().map(|h| h.1).collect();
    println!("{:#?}", whale.to_string());
    create_graphic_2d(
        "graphics/woa/graphic_f9.png",
        "F9(X) = Sum(i = 1..n, -X[i].powf(2) - 10 * (2 * PI * X[i]).cos() + 10)",
        (0, 499),
        (0f64, 20f64),
        &all_fitness[..]
    ).expect("Could not create graphic 2D")
    /*let population_size: usize = 40;
    let dim = 3;
    //Lower, Upper
    let bounds: (f64, f64) = (-500f64, 500f64);
    let maximization: bool = false;
    let max_iterations: usize = 500;
    let mut algorithm: WOA = WOA::initialize(
        dim,
        population_size,
        bounds,
        maximization,
        max_iterations,
        f_8,
    );
    let history = algorithm.run();
    let whale = Whale::from_fn(|| history.last().unwrap().clone());
    let all_fitness: Vec<f64> = history.iter().map(|h| h.1).collect();
    println!("{:#?}", whale.to_string());
    create_graphic_2d(
        "graphics/woa/graphic_f8.png",
        "F9(X) = Sum(i = 1..n, -X[i] * (-X[i].abs().sqrt()).sin())",
        (0, 499),
        (-400f64, -420f64),
        &all_fitness[..]
    ).expect("Could not create graphic 2D")*/
}
