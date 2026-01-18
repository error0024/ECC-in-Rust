use num_bigint::BigUint;
use num_prime::nt_funcs::is_prime;

pub struct FiniteField {}

impl FiniteField {
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c+ d mod p
        let r: BigUint = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c* d mod p
        let r: BigUint = c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        if c < p {
            return p - c;
        }
        let r = c.modpow(&BigUint::from(1u32), p);
        p - r
    }

    pub fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c - d mod p
        let r: BigUint = FiniteField::inv_addition(d, p);
        FiniteField::add(c, &r, p)
    }

    pub fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // only works for prime p
        // c^(-1) mod p
        assert!(is_prime(p, None).probably()); // use default primality check config
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    pub fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        FiniteField::mult(c, &FiniteField::inv_multiplication(d, p), p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_0() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::add(&a, &b, &p), BigUint::from(3u32));
    }

    #[test]
    fn test_add_1() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(15u32);
        assert_eq!(FiniteField::add(&a, &b, &p), BigUint::from(14u32));
    }

    #[test]
    fn test_mul_0() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::mult(&a, &b, &p), BigUint::from(7u32));
    }

    #[test]
    fn test_mul_1() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(47u32);
        assert_eq!(FiniteField::mult(&a, &b, &p), BigUint::from(40u32));
    }

    #[test]
    fn test_inv_add_0() {
        let a = BigUint::from(7u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::inv_addition(&a, &p), BigUint::from(4u32));
    }

    #[test]
    fn test_inv_add_1() {
        let a = BigUint::from(15u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::inv_addition(&a, &p), BigUint::from(7u32));
    }

    #[test]
    fn test_inv_add_2() {
        let a = BigUint::from(15u32);
        let p = BigUint::from(11u32);
        let c = FiniteField::inv_addition(&a, &p);
        assert_eq!(FiniteField::add(&a, &c, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_inv_mult_0() {
        let a = BigUint::from(1u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::inv_multiplication(&a, &p), BigUint::from(1u32));
    }

    #[test]
    fn test_inv_mult_1() {
        let a = BigUint::from(2u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::inv_multiplication(&a, &p), BigUint::from(6u32));
    }

    #[test]
    fn test_inv_mult_2() {
        let a = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        assert_eq!(FiniteField::inv_multiplication(&a, &p), BigUint::from(10u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_mult_3() {
        let a = BigUint::from(1u32);
        let p = BigUint::from(10u32);
        FiniteField::inv_multiplication(&a, &p);
    }

    #[test]
    fn test_inv_mult_4() {
        let a = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::inv_multiplication(&a, &p);
        assert_eq!(FiniteField::mult(&a, &r, &p), BigUint::from(1u32));
    }
}
