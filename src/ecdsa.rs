use crate::{EllipticCurve, Point, FiniteField};
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};
//use ec_generic::{EllipticCurve, Point, FiniteField};

pub struct ECDSA{
    elliptic_curve: EllipticCurve, //curve that we are working with
    a_gen: Point, //generator of the curve group
    q_order: BigUint, //order of gen
}

impl ECDSA {
    pub fn new(elliptic_curve: EllipticCurve, a_gen: Point, q_order: BigUint) -> Self {
        ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        }
    }

    //Generated: d, B = d*A
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let priv_key: BigUint = self.generate_priv_key();
        let pub_key: Point = self.generate_public_key(&priv_key);
        (priv_key, pub_key)
    }

    
    pub fn sign(&self, hash: &BigUint, priv_key: &BigUint) -> (BigUint, BigUint) {
        todo!()
    }

    pub fn verify(&self, hash: &BigUint, pub_key: &Point, signature: &(BigUint, BigUint)) -> bool {
        todo!()
    }

    //(not pub functions, since they are only for internal use)
    fn generate_priv_key(&self) -> BigUint {
        generate_random_positive_number_less_than(&self.q_order)
    }

    fn generate_public_key(&self, private_key: &BigUint) -> Point {
        self.elliptic_curve.scalar_mul(&self.a_gen, private_key)
    }
}

//[1, max) 
fn generate_random_positive_number_less_than(max: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let num = rng.gen_biguint_range(&BigUint::from(1u32), max);
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secp256k1() -> (EllipticCurve, Point, BigUint) {
        let ecc = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p: BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap(),
        };
        let gx = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
        let gy = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
        let generator = Point::Coordinate(gx, gy);
        let order = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
        (ecc, generator, order)
    }

    #[test]
    fn test_rng() {
        let max = BigUint::from(10000u32);
        let num = generate_random_positive_number_less_than(&max);
        //print!("Generated number: {}", num);
        assert!(num < max, "Expected {} to be less than {}", num, max);
    }

    #[test]
    fn test_key_gen() {
        let (ecc, generator, order) = secp256k1();
        let ecdsa = ECDSA::new(ecc.clone(), generator.clone(), order);
        let key_pair = ecdsa.generate_key_pair();
        assert_eq!(ecc.scalar_mul(&generator, &key_pair.0), key_pair.1);
    }
}
