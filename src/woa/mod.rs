use crate::whale::{Whale};
use na::DVector;
use rand::{Rng, rng};
use std::cell::RefCell;
use std::f64::consts::{E, PI};
use std::ops::Add;
use std::rc::Rc;

pub type ObjectiveFunction = fn(&DVector<f64>) -> f64;
#[derive(Debug)]
pub struct WOA {
    whales: Vec<Rc<RefCell<Whale>>>,
    best_whale: Rc<RefCell<Whale>>,
    bounds: (f64, f64),
    maximization: bool,
    max_iterations: usize,
    objective_function: ObjectiveFunction,
}

impl WOA {
    pub fn initialize(
        dim: usize,
        population_size: usize,
        bounds: (f64, f64),
        maximization: bool,
        max_iterations: usize,
        obj_func: ObjectiveFunction,
    ) -> Self {
        let whales = Self::initialize_whales(dim, population_size, bounds.0, bounds.1);
        Self {
            whales,
            best_whale: Rc::new(RefCell::new(Whale::zeros(dim))),
            objective_function: obj_func,
            bounds,
            maximization,
            max_iterations,
        }
    }

    fn initialize_whales(
        dim: usize,
        population_size: usize,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Vec<Rc<RefCell<Whale>>> {
        let mut whales: Vec<Rc<RefCell<Whale>>> = vec![];
        for _ in 0..population_size {
            whales.push(Rc::new(RefCell::new(Whale::with_random_components(
                dim,
                lower_bound,
                upper_bound,
            ))));
        }
        whales
    }

    fn calculate_fitness_for_each_whale(&self) {
        for whale in &self.whales {
            whale.borrow_mut().calculate_fitness(self.objective_function);
        }
    }

    fn sort_whales_asc(&mut self) {
        self.whales
            .sort_by(|a, b| a.borrow().fitness.total_cmp(&b.borrow().fitness));
    }

    fn sort_whales_desc(&mut self) {
        self.whales
            .sort_by(|a, b| b.borrow().fitness.total_cmp(&a.borrow().fitness))
    }

    fn sort_whales(&mut self) {
        if self.maximization {
            self.sort_whales_desc()
        } else {
            self.sort_whales_asc()
        }
    }
    pub fn run(&mut self) -> Vec<(DVector<f64>, f64)> {
        self.calculate_fitness_for_each_whale();
        self.sort_whales();
        self.best_whale
            .replace_with(|_| self.whales[0].borrow().clone());
        let dim = self.best_whale.borrow().position.len();
        let mut history: Vec<(DVector<f64>, f64)> = vec![];
        for t in 0..self.max_iterations {
            let a: f64 = 2f64 - t as f64 * 2f64 / self.max_iterations as f64;
            for w in &self.whales {
                let r: DVector<f64> = DVector::from_fn(dim, |_, _| {
                    rng().random_range(self.bounds.0..=self.bounds.1)
                });
                let coefficient_a: DVector<f64> = (&r * a).add_scalar(a);
                let coefficient_c: DVector<f64> = 2f64 * r;
                let l: f64 = rng().random_range(-1f64..=1f64);
                let p: f64 = rng().random_range(0f64..=1f64);
                let b_w: DVector<f64> = self.best_whale.borrow().position.clone();
                if p < 0.5 {
                    if coefficient_a.norm() < 1f64 {
                        let d_coefficient: DVector<f64> =
                            (coefficient_c.component_mul(&b_w) - &w.borrow().position).abs();
                        w.borrow_mut().position =
                            &b_w - coefficient_a.component_mul(&d_coefficient);
                        continue;
                    }
                    let random_pos: usize =
                        rng().random_range(0f64..self.whales.len() as f64) as usize;
                    let r_whale: DVector<f64> = self.whales[random_pos].borrow().position.clone();
                    let d_coefficient: DVector<f64> =
                        (coefficient_c.component_mul(&r_whale) - &w.borrow().position).abs();
                    w.borrow_mut().position = r_whale - coefficient_a.component_mul(&d_coefficient);
                    continue;
                }
                let d_coefficient: DVector<f64> = (&b_w - &w.borrow().position).abs();
                w.borrow_mut().position = b_w.add(d_coefficient * E.powf(l) * (2f64 * PI * l).cos())
            }
            self.check_if_any_whale_goes_beyond_bounds();
            self.calculate_fitness_for_each_whale();
            self.sort_whales();
            let current_gen_best = self.whales[0].clone();
            let curr_best_fit = self.best_whale.borrow().fitness;
            let curr_gen_best_fit = current_gen_best.borrow().fitness;
            if curr_gen_best_fit < curr_best_fit && !self.maximization {
                self.best_whale
                    .replace_with(|_| current_gen_best.borrow().clone());
            } else if curr_gen_best_fit > curr_best_fit && self.maximization {
                self.best_whale
                    .replace_with(|_| current_gen_best.borrow().clone());
            }
            history.push(self.best_whale.borrow().as_tuple())
        }
        history
    }

    fn check_if_any_whale_goes_beyond_bounds(&self) {
        for w in &self.whales {
            let position: &mut DVector<f64> = &mut w.borrow_mut().position;
            for i in 0..position.len() {
                if position[i] < self.bounds.0 {
                    position[i] = self.bounds.0;
                } else if position[i] > self.bounds.1 {
                    position[i] = self.bounds.1;
                }
            }
        }
    }
}
