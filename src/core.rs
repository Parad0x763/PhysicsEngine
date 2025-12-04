use std::{io::Error, ops::{Add, Mul, Div, Sub, Rem, RemAssign}};

const DEFAULT: f32 = 0.0;
const NEGATION: f32 = -1.0;

/// Holds a vector of three dimenions.
/// Four members are allocated to ensure alignment in an array.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pad: f32, // padding to ensure four word alingnment
}

impl Default for Vector3 {
    fn default() -> Self {
        return Self {
            x: DEFAULT,
            y: DEFAULT,
            z: DEFAULT,
            pad: DEFAULT
        };
    }
}

// OPERATOR OVERLOADS

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        return Vector3 {
            x: _rhs * self.x,
            y: _rhs * self.y,
            z: _rhs * self.z,
            pad: self.pad
        };
    }
}

impl Mul<&Vector3> for Vector3 {
    type Output = f32;
    /// Used to calculate the Scalar Product between two vectors
    fn mul(self, _rhs: &Vector3) -> f32 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {
        return Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            pad: self.pad
        };
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: f32) -> Vector3 {
        return Vector3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
            pad: self.pad
        };
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: &Vector3) -> Vector3 {
        return Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            pad: self.pad
        };
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: f32) -> Vector3 {
        return Vector3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
            pad: self.pad
        };
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: &Vector3) -> Vector3 {
        return Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            pad: self.pad
        };
    }
}

impl RemAssign<&Vector3> for &mut Vector3 {
    fn rem_assign(&mut self, _rhs: &Vector3) {
        let temp: Vector3 = Vector3::new(self.x, self.y, self.z);
        self.update_by_vector3( temp.vector_product(_rhs) );
    }
}

impl Rem<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn rem(self, _rhs: &Vector3) -> Vector3 {
        return self.vector_product(_rhs);
    }
}

impl Rem<&Vector3> for &mut Vector3 {
    type Output = Vector3;
    
    fn rem(self, _rhs: &Vector3) -> Vector3 {
        return self.vector_product(_rhs);
    }
}

// `Vector3` IMLEMENTATION

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self {
            x,
            y,
            z,
            pad: DEFAULT,
        };
    }

    /// Inverts the Vector by flipping the signs of each axis, x, y and z
    pub fn invert(&mut self) {
        self.x *= NEGATION;
        self.y *= NEGATION;
        self.z *= NEGATION;
    }

    /// Returns the magnitude of the vector: Square Root of the Sum of the square of each axis
    pub fn magnitude(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    /// Returns the square magnitude: Sum of the squares of each axis
    pub fn square_magnitude(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    /// Returns the square magnitude: Sum of the squares of each axis. Takes a mut ref of self.
    pub fn square_magnitude_mut(&mut self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn update_by_vector3(&mut self, other: Vector3) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    /// Normalizes the vector.
    /// To normalize a Vector, you multiply each axis of the vector by the vector,
    /// then dived by the magnitude of the vector.
    pub fn normalize(&mut self) {
        let length: f32 = self.magnitude();
        // for vector a, you dived each component by the magnitude of vector a
        self.update_by_vector3(
            Vector3 { x: self.x, y: self.y, z: self.z, pad: self.pad } / length
        );
    }

    /// Cacluates and returns a component-wise product by multiplying self * other.
    /// Returns a new `Vector3` with the results.
    pub fn component_product(&self, other: &Vector3) -> Self {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            pad: self.pad
        }
    }

    /// Performs a component-wise product with the given vector
    /// and sets this vector to its result.
    pub fn component_product_update(&mut self, other: &Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }

    /// Calculates and returns the vector product of this vector with the given other vector.
    pub fn vector_product(self, other: &Vector3) -> Vector3 {
        return Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            pad: self.pad
        };
    }

    /// Determines if an Orthonormal Basis exists between the three Vectors.
    /// Finds Vector c by performing the cross-product: `c = a X b`.
    /// If c has a zero magnitude then a and b are parallel.
    /// Then ensure that a and b are right angles to each other using the cross product: `b = c X a`
    pub fn make_orthonormal_basis(a: &mut Vector3, b: &mut Vector3, c: &mut Vector3) -> Result<(), Error> {
        a.normalize();

        c.update_by_vector3(
            a.rem(b)
        );

        if c.square_magnitude_mut() == 0.0 {
            return Err(Error::new(std::io::ErrorKind::Other, "Vector a is parallel to Vector b"));
        }

        c.normalize();
        b.update_by_vector3(
            c.rem(a)
        );
        return Ok(());
    }
}
