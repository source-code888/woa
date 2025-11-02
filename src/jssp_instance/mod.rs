use na::{DMatrix, DVector, SimdValue};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Error, Read};

#[derive(Debug, PartialEq)]
pub enum Instance {
    LA05,
    // I took this instance from:
    // Hybrid Whale Optimization Algorithm Enhanced with Lévy Flight and Differential Evolution
    // WOA-LFDE
    // https://doi.org/10.1016/j.asoc.2019.105954
    TEST03,
}

impl Instance {
    pub fn to_string(&self) -> String {
        match self {
            Instance::LA05 => String::from("la05"),
            Instance::TEST03 => String::from("test03"),
        }
    }
}

#[derive(Debug, PartialEq)]
/// # JSSP Instance
/// This type represents a JSSP instance, we read an instance from .txt files
/// which we obtained from the literature.
/// - **instance** is the name of the instance, I decided to put it in an enum type
/// because this way we know which instances we have in **src/jssp_instance/lit/**
/// - **sequences** represents the sequence in which each operation of given jobs are to be processed.
/// - **processing_times** represents the time thah each job operation take in a given machine.
/// - **n** is the number of jobs.
/// - *m** is the number of machines.
/// Each vector used here is *0-indexed*, so when we create a *schedule* and we have 3 jobs, our schedule
/// should look like this:
/// 0 1 0 2 0 1 1 2
/// 0 represents **job 1**, 1 represents **job 2** and 2 represents **job 3**.
pub struct JSSPInstance {
    pub instance: Instance,
    pub sequences: Vec<Vec<usize>>,
    pub processing_times: Vec<Vec<u32>>,
    pub n: u32,
    pub m: u32,
}

impl JSSPInstance {
    fn new(
        instance: Instance,
        sequences: Vec<Vec<usize>>,
        processing_times: Vec<Vec<u32>>,
        n: u32,
        m: u32,
    ) -> Self {
        Self {
            instance,
            sequences,
            processing_times,
            n,
            m,
        }
    }

    pub fn from_instance(instance: Instance, path: String) -> Result<Self, Error> {
        let path = format!("{path}/{}.txt", instance.to_string());
        let content: String = Self::read_file(path)?;
        let mut lines = Self::get_lines(content);
        let first_line = lines.remove(0);
        let n: u32 = first_line[0];
        let m: u32 = first_line[1];
        let sequences = Self::read_sequences(&lines);
        let processing_times = Self::read_processing_times(&lines, &sequences);
        Ok(Self::new(instance, sequences, processing_times, n, m))
    }

    fn read_sequences(lines: &Vec<Vec<u32>>) -> Vec<Vec<usize>> {
        let mut sequences: Vec<Vec<usize>> = vec![];
        for l in lines {
            let mut i: usize = 0;
            let mut aux: Vec<usize> = vec![];
            while i < l.len() {
                aux.push(l[i] as usize);
                i += 2;
            }
            sequences.push(aux);
        }
        sequences
    }

    fn read_processing_times(lines: &Vec<Vec<u32>>, sequences: &Vec<Vec<usize>>) -> Vec<Vec<u32>> {
        let mut processing_times: Vec<Vec<u32>> = vec![];
        for i in 0..lines.len() {
            let l = &lines[i];
            let sequence = &sequences[i];
            let mut aux: Vec<u32> = Vec::from_iter(0..sequence.len() as u32);
            let mut j = 1usize;
            for next_index in sequence {
                aux[*next_index] = l[j];
                j += 2;
            }
            processing_times.push(aux);
        }
        processing_times
    }

    fn get_lines(content: String) -> Vec<Vec<u32>> {
        content
            .lines()
            .map(|l| {
                l.replace("\t", " ")
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn read_file(path: String) -> Result<String, Error> {
        let file = File::open(path)?;
        let mut content: String = String::new();
        BufReader::new(file).read_to_string(&mut content)?;
        Ok(content)
    }

    /// # Makespan
    /// If you want to know how to calculate **makespan** of a given **schedule**, or **processing sequence**
    /// you should read the literature, the **operations** are **as they appear** in the squedule.
    pub fn calculate_makespan(&self, schedule: &DVector<usize>) -> u32 {
        let mut operations: DVector<usize> = DVector::zeros(self.n as usize);
        let mut job_time: DVector<u32> = DVector::zeros(self.n as usize);
        let mut machine_time: DVector<u32> = DVector::zeros(self.m as usize);
        for job in schedule {
            let job: usize = *job;
            // Machine of given operation of the current job
            let machine: usize = self.sequences[job][operations[job]];
            let start = job_time[job].max(machine_time[machine]);
            let finish = start + self.processing_times[job][machine];
            job_time[job] = finish;
            machine_time[machine] = finish;
            operations[job] += 1;
        }
        job_time.max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let instance: Instance = Instance::TEST03;
        let instance: Result<JSSPInstance, Error> =
            JSSPInstance::from_instance(instance, String::from("src/jssp_instance/lit/"));
        assert!(instance.is_ok());
        let instance = instance.unwrap();
        assert_eq!(
            instance,
            JSSPInstance::new(
                Instance::TEST03,
                vec![vec![0, 2, 1], vec![1, 0, 2], vec![2, 1, 0]],
                vec![vec![10, 5, 15], vec![8, 15, 20], vec![15, 10, 9]],
                3,
                3
            )
        );
        let schedule: DVector<usize> = DVector::from_vec(vec![0, 1, 1, 2, 0, 2, 1, 0, 2]);
        assert_eq!(instance.calculate_makespan(&schedule), 45)
    }
}
