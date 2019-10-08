use crate::vector::Vector;
use rand::Rng;

//List of random vectors that are generated to be used in calculating the
//'random' diffuse reflections. 'Random' ray is calculated by adding normalized
//normal vector and one of the vector.
pub struct Lambertian{
    vectors: Vec<Vector>
}

impl Lambertian{
    pub fn new(num_of_vectors: usize) -> Lambertian{
        let mut vectors = Vec::with_capacity(num_of_vectors);
        let mut rng = rand::thread_rng();
        for idx in 0..num_of_vectors{
            let x: f64 = rng.gen_range(-1., 1.);
            let y_bounds = (1. - x.powi(2)).sqrt();
            let y: f64 = rng.gen_range(-y_bounds, y_bounds);
            let z: f64;
            //Calculate 50% positive z values and 50% negative
            if rng.gen_bool(0.5){
                z = (1. - x.powi(2) - y.powi(2)).sqrt();
            }
            else {
                z = -(1. - x.powi(2) - y.powi(2)).sqrt();
            }
            vectors.push(Vector{
                x,
                y,
                z
            });
        }
        Lambertian{
            vectors
        }
    }
    pub fn get_offsets(&self) -> &Vec<Vector>{
        &self.vectors
    }
}

#[cfg(test)]
mod test{
    use crate::lambertian::Lambertian;
    use crate::vector::Vector;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn generate_vectors()
    {
        let lambertian = Lambertian::new(100);
        assert_eq!(100, lambertian.get_offsets().len());
        for offset in lambertian.get_offsets().iter()
        {
            assert_approx_eq!(1., offset.distance());
        }
    }
}