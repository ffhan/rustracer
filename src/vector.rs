use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn zero() -> Vector {
        Vector {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {x: x, y: y, z: z}
    }
    pub fn from_array(arr: [f64; 3]) -> Vector {
        Vector {
            x: arr[0],
            y: arr[1],
            z: arr[2]
        }
    }
    pub fn from_vec(vec: Vec<f64>) -> Vector {
        if vec.len() != 3 {
            panic!("Invalid vec size {}", vec.len());
        }
        let mut arr = [0.0_f64, 0.0, 0.0];
        vec.into_iter().enumerate().for_each(|(i, e)| arr[i] = e);
        Vector::from_array(arr)
    }
    pub fn normalize(&self) -> Vector {
        let distance = self.euclidian_distance();
        Vector::new(self.x / distance, self.y / distance, self.z / distance)
    }
    pub fn euclidian_distance(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
    pub fn plus(&self, vec: &Vector) -> Vector {
        Vector::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
    pub fn minus(&self, vec: &Vector) -> Vector {
        Vector::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
    pub fn neg(&self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
    pub fn dot(&self, vec: &Vector) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }
    pub fn cross(&self, vec: &Vector) -> Vector {
        Vector::new(
            self.y * vec.z - self.z * vec.y,
            -(self.x * vec.z - self.z * vec.x),
            self.x * vec.y - self.y * vec.x
        )
    }
    pub fn factor(&self, factor: f64) -> Vector {
        Vector::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        self.plus(&rhs)
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.dot(&rhs)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}
