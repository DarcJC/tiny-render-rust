use std::fs::read_to_string;
use std::str::FromStr;
use std::fmt;

pub struct ModelObject {
    vertices: Vec<(f64, f64, f64)>,
    faces: Vec<(usize, usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct ModelParseError;

impl fmt::Display for ModelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse model")
    }
}

impl ModelObject {
    pub fn from_file(filename: &str) -> Result<Self, ModelParseError> {
        let contents = read_to_string(filename).unwrap_or("".to_string());
        let lines: Vec<&str> = contents.split("\r\n").collect();
        {
            let mut t = Self {
                vertices: vec![],
                faces: vec![]
            };
            for (i, line) in lines.iter().enumerate() {
                if line.len() < 3 {
                    continue;
                }
                let start_pattern = &line[0..2];
                let pure_data = &line[2..];
                if let "v " = start_pattern {
                    let pos: Vec<&str> = pure_data.split(' ').collect();
                    if pos.len() != 3 {
                        println!("Bad file line: {}", i + 1);
                        continue;
                    }
                    let mut c = [0f64; 3];
                    for (i, d) in pos.iter().enumerate() {
                        match f64::from_str(d) {
                            Ok(n) => c[i] = n,
                            Err(_e) => return Err(ModelParseError),
                        }
                    }
                    t.vertices.push((c[0], c[1], c[2]));
                } else if let "f " = start_pattern {
                    let v_meta: Vec<&str> = pure_data.split(' ').collect();
                    if v_meta.len() != 3 {
                        println!("Bad file line: {}", i + 1);
                        continue;
                    }
                    let mut c = [0usize; 3];
                    for (i, element) in v_meta.iter().enumerate() {
                        let meta: Vec<&str> = element.split("/").collect();
                        match usize::from_str(meta[0]) {
                            Ok(n) => c[i] = n,
                            Err(_e) => return Err(ModelParseError),
                        }
                    }
                    t.faces.push((c[0], c[1], c[2]));
                }
            }
            Ok(t)
        }
    }
    pub fn count_vertices(&self) -> usize {self.vertices.len()}
    pub fn count_faces(&self) -> usize {self.faces.len()}
    pub fn get_vertex(&self, index: usize) -> (f64, f64, f64) {
        self.vertices[index]
    }
    pub fn get_face(&self, index: usize) -> (usize, usize, usize) {
        self.faces[index]
    }
}
