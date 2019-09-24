use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vector
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Vector> for Vector
{
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector
    {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Vector
{
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector
    {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector
{
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector
    {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector> for Vector
{
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector
    {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Vector
{
    pub fn distance(&self) -> f64
    {
        let sum_of_squares = self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.);
        return sum_of_squares.sqrt();
    }

    pub fn dot(&self, other: Vector) -> f64
    {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

#[cfg(test)]
mod tests
{
    use crate::vector::Vector;
    use assert_approx_eq::assert_approx_eq;

    fn get_test_vectors() -> (Vector, Vector)
    {
        let vec_a = Vector {
            x: 2.1,
            y: 3.4,
            z: 5.7,
        };

        let vec_b = Vector {
            x: -2.2,
            y: 4.9,
            z: 7.4,
        };
        (vec_a, vec_b)
    }

    #[test]
    fn add()
    {
        let (vec_a, vec_b) = get_test_vectors();

        let result_vec = vec_a + vec_b;
        assert_approx_eq!(result_vec.x, -0.1);
        assert_approx_eq!(result_vec.y, 8.3);
        assert_approx_eq!(result_vec.z , 13.1);
    }

    #[test]
    fn sub()
    {
        let (vec_a, vec_b) = get_test_vectors();

        let result_vec = vec_a - vec_b;
        assert_approx_eq!(result_vec.x, 4.3);
        assert_approx_eq!(result_vec.y, -1.5);
        assert_approx_eq!(result_vec.z , -1.7);
    }

    #[test]
    fn mul_scalar()
    {
        let (vec_a, _vec_b) = get_test_vectors();

        let result_vec = vec_a * 7.;
        assert_approx_eq!(result_vec.x, 14.7, 1e-5);
        assert_approx_eq!(result_vec.y, 23.8, 1e-5);
        assert_approx_eq!(result_vec.z , 39.9, 1e-5);
    }

    #[test]
    fn mul_cross()
    {
        let (vec_a, vec_b) = get_test_vectors();

        let result_vec = vec_a * vec_b;
        assert_approx_eq!(result_vec.x, -2.77, 1e-5);
        assert_approx_eq!(result_vec.y, -28.08, 1e-5);
        assert_approx_eq!(result_vec.z , 17.77 , 1e-5);
    }

    #[test]
    fn distance()
    {
        let (vec_a, _vec_b) = get_test_vectors();

        let distance = vec_a.distance();
        assert_approx_eq!(distance , 6.9613, 1e-4);
    }
}