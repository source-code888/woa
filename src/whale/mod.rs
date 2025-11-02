use na::DVector;
use rand::{Rng, rng};

#[derive(Debug, Clone)]
pub struct Whale {
    pub position: DVector<f64>,
    pub fitness: f64,
}

impl Whale {
    /// Creates a new whale with a given dimension
    pub fn zeros(dim: usize) -> Self {
        Self {
            position: DVector::zeros(dim),
            fitness: 0f64,
        }
    }

    pub fn with_random_components(dim: usize, lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            position: DVector::from_fn(dim, |_, _| rng().random_range(lower_bound..=upper_bound)),
            fitness: 0f64,
        }
    }

    /// Calculates fitness for an individual using a given objective function
    pub fn calculate_fitness<F>(&mut self, func: F)
    where F: Fn(&DVector<f64>) -> f64 {
        self.fitness = func(&self.position);
    }

    pub fn to_string(&self) -> String {
        format!(
            "Whale {{position: {:?}, fitness: {}}}",
            self.position.data.as_vec(),
            self.fitness
        )
    }

    pub fn as_tuple(&self) -> (DVector<f64>, f64) {
        (self.position.clone(), self.fitness)
    }

    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn() -> (DVector<f64>, f64),
    {
        let val: (DVector<f64>, f64) = f();
        Self {
            position: val.0,
            fitness: val.1,
        }
    }
}
