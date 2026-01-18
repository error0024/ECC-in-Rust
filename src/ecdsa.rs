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
        self.generate_random_positive_number_less_than(&self.q_order)
    }

    fn generate_public_key(&self, private_key: &BigUint) -> Point {
        todo!()
    }
    //[1, max) 
    fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        todo!();
    }
}

