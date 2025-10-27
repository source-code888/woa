use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};

#[derive(Debug)]
pub enum Instance {
    LA05
}

impl Instance {
    pub fn to_string(&self) -> String {
        match self {
            Instance::LA05 => String::from("la05"),
        }
    }
}

pub struct JSSPInstance {
    instance: Instance,
    sequences: Vec<Vec<usize>>,
    processing_times: Vec<Vec<u32>>
}

impl JSSPInstance {
    fn new(instance: Instance, sequences: Vec<Vec<usize>>, processing_times: Vec<Vec<u32>>) -> Self {
        Self {
            instance,
            sequences,
            processing_times,
        }
    }

    pub fn from_instance(instance: Instance, path: String) -> Result<Self, Error> {
        let instance_name_with_ext = format!("{}.txt", instance.to_string());
        let path = format!("{path}/{instance_name_with_ext}");
        let file = File::open(path)?;
        let mut content: String = String::new();
        BufReader::new(file).read_to_string(&mut content)?;
        println!("{content}");
        Err(Error::new(ErrorKind::InvalidInput, "Invalid input file"))
    }
}

