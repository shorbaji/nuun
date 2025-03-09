use std::ops;

#[derive(Clone, Debug)]
pub enum Path {
    Relative(Vec<String>),
    Absolute(Vec<String>),
}

impl ops::Add<&str> for Path {
    type Output = Path;

    fn add(self, rhs: &str) -> Self::Output {
        match self {
            Path::Relative(v) => {
                let mut v = v.clone();
                v.push(rhs.to_string());
                Path::Relative(v)
            },
            Path::Absolute(v) => {
                let mut v = v.clone();
                v.push(rhs.to_string());
                Path::Absolute(v)
            }
        }
    }
}


impl ops::Add<&Path> for Path {
    type Output = Path;

    fn add(self, rhs: &Path) -> Self::Output {
        let a = self.as_vector();
        let b = rhs.as_vector();

        let c = a.iter().chain(b.iter()).cloned().collect();

        match self {
            Path::Absolute(_) => Path::Absolute(c),
            Path::Relative(_) => Path::Relative(c),
        }
    }
}

impl ops::Sub<&Path> for Path {
    type Output = Path;

    fn sub(self, rhs: &Path) -> Self::Output {
        let a = self.as_vector();
        let b = rhs.as_vector();

        let c = a.iter().zip(b.iter()).skip_while(|(a, b)| a == b).map(|(a, _)| a).cloned().collect();

        Path::Relative(c)
    }
}

impl Path {
    fn includes(&self, other: &Path) -> bool {
        match (self, other) {
            (Path::Absolute(a), Path::Absolute(b)) => {
                a.len() >= b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
            }
            (Path::Relative(a), Path::Relative(b)) => {
                a.len() >= b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
            }
            _ => false,
        }
    }

    pub fn as_vector(&self) -> Vec<String> {
        match self {
            Path::Absolute(v) => v.clone(),
            Path::Relative(v) => v.clone(),
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = self.as_vector();
        write!(f, "{}", v.join("/"))
    }
}