use num_bigint::{BigUint};
use num_prime::nt_funcs::is_prime;

struct FiniteField {}

#[derive(Clone, PartialEq, Debug)]
enum Point{
    Coordinate(BigUint, BigUint),
    Identity,
}
struct EllipticCurve{
    //y^2 = x^2 + a*x + b in F_p
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl FiniteField {
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c+ d mod p
        let r: BigUint =  c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c+ d mod p
        let r: BigUint =  c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        if c < p {
            return p-c;
        }
        let r = c.modpow(&BigUint::from(1u32), p);
        return p-r;
    }

    pub fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        //r = c+ d mod p
        let r: BigUint =  FiniteField::inv_addition(&d, &p);
        FiniteField::add(c, &r, &p)
    }

    pub fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // only works for prime p
        // c^(-1) mod p
        assert!(is_prime(p, None).probably()); // use default primality check config
        let r = c.modpow(&(p - BigUint::from(2u32)), p);
        return r;
    }

    pub fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        FiniteField::mult(&c, &FiniteField::inv_multiplication(&d, &p), &p)
    }

}

impl EllipticCurve {
    fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not a curve!");
        assert!(self.is_on_curve(d), "Point is not a curve!");
        if *c == *d {
            return self.double(c);
        }
        match (c,d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                // s = (y2-y1/x2-x1) mod p
                // x = s^2 - x1 -x2 mod p
                // y = s* (x1 - x) - y1 mod p
                let s = FiniteField::divide(
                    &FiniteField::subtract(y2, y1, &self.p),
                    &FiniteField::subtract( x2, x1, &self.p), &self.p);
                let x = FiniteField::subtract(&FiniteField::subtract(&s.modpow(&BigUint::from(2u32), &self.p), x1, &self.p), &x2, &self.p);
                let y =  FiniteField::subtract(&FiniteField::mult(&s, &FiniteField::subtract(x1, &x, &self.p),  &self.p), y1,  &self.p);
                assert!(self.is_on_curve(&Point::Coordinate(x.clone(), y.clone())), "addition went wrong");
                return Point::Coordinate(x, y);
            }
            
        }
    }

    fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not a curve!");
        todo!()
    }

    fn scalar_mul(&self, c: &Point, d: &BigUint ) -> Point {
        //addition/doubling algorithm
        assert!(self.is_on_curve(c), "Point is not a curve!");
        todo!()
    }
    
    fn is_on_curve(&self, c: &Point) -> bool {
        //y^2 = x^3 + ax + b
        match c {
            Point::Coordinate(x, y ) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, &x, &self.p);
                let axb =  FiniteField::add(&ax, &self.b, &self.p);
                return y2 == FiniteField::add(&x3,  &axb, &self.p)
            },
            Point::Identity => true
        }
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
    #[test]
    fn test_is_on_curve_0(){
        let ecc = EllipticCurve{
            //y^2 = x^3 + a*x + b in F_p
            a: BigUint::from(1u32),
            b: BigUint::from(2u32),
            p: BigUint::from(11u32),
        };
        let x = BigUint::from(1u32);
        let y = BigUint::from(2u32);
        assert_eq!(ecc.is_on_curve(&Point::Coordinate(x, y)), true);
    }

    #[test]
    fn test_ec_point_addition_0(){
        let ecc = EllipticCurve{
            //y^2 = x^3 + 2*x + 2 in F_17
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);
        assert_eq!(ecc.is_on_curve(&r), true);

        let res = ecc.add(&p1, &p2);
        assert_eq!(res, r);
        print!("{:?}", res);    
    }

    #[test]
    fn test_ec_point_addition_1(){
        let ecc = EllipticCurve{
            //y^2 = x^3 + 2*x + 2 in F_17
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);

        let res = ecc.add(&p1, &p2);
        assert_eq!(res, p1);
        print!("{:?}", res);    
    }

    #[test]
    fn test_ec_point_addition_3(){
        let ecc = EllipticCurve{
            //y^2 = x^3 + 2*x + 2 in F_17
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);

        let res = ecc.add(&p2, &p1);
        assert_eq!(res, p1);
        print!("{:?}", res);    
    }
}