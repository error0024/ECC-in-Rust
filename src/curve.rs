use num_bigint::BigUint;

use crate::field::FiniteField;

#[derive(Clone, PartialEq, Debug)]
pub enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}

#[derive(Clone)]
pub struct EllipticCurve {
    //y^2 = x^2 + a*x + b in F_p
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
}

impl EllipticCurve {
    fn coord_add(&self, s: &BigUint, x1: &BigUint, y1: &BigUint, x2: &BigUint, y2: &BigUint) -> Point {
        let x = FiniteField::subtract(&FiniteField::subtract(&s.modpow(&BigUint::from(2u32), &self.p), x1, &self.p), x2, &self.p);
        let y = FiniteField::subtract(&FiniteField::mult(s, &FiniteField::subtract(x1, &x, &self.p), &self.p), y1, &self.p);
        assert!(self.is_on_curve(&Point::Coordinate(x.clone(), y.clone())), "addition went wrong");
        Point::Coordinate(x, y)
    }

    pub fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not on curve!");
        assert!(self.is_on_curve(d), "Point is not on curve!");
        if *c == *d {
            return self.double(c);
        }
        match (c, d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                if x1 == x2 {
                    return Point::Identity;
                }
                // s = (y2-y1)/(x2-x1) mod p
                // x = s^2 - x1 - x2 mod p
                // y = s * (x1 - x) - y1 mod p
                let s = FiniteField::divide(
                    &FiniteField::subtract(y2, y1, &self.p),
                    &FiniteField::subtract(x2, x1, &self.p),
                    &self.p,
                );
                self.coord_add(&s, x1, y1, x2, y2)
            }
        }
    }

    pub fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not on curve!");
        match c {
            Point::Identity => Point::Identity,
            Point::Coordinate(x1, y1) => {
                // Special case: if y1 == 0, the tangent is vertical -> result is Identity
                if *y1 == BigUint::from(0u32) {
                    return Point::Identity;
                }
                // s = (3*x1^2 + a) / (2*y1) mod p
                // x = s^2 - x1 - x1 mod p
                // y = s * (x1 - x) - y1 mod p
                let s = FiniteField::divide(
                    &FiniteField::add(
                        &FiniteField::mult(&BigUint::from(3u32), &x1.modpow(&BigUint::from(2u32), &self.p), &self.p),
                        &self.a,
                        &self.p,
                    ),
                    &FiniteField::mult(&BigUint::from(2u32), y1, &self.p),
                    &self.p,
                );
                self.coord_add(&s, x1, y1, x1, y1)
            }
        }
    }

    pub fn scalar_mul(&self, c: &Point, d: &BigUint) -> Point {
        assert!(self.is_on_curve(c), "Point is not on curve!");
        if d == &BigUint::from(0u32) || *c == Point::Identity {
            Point::Identity
        } else if d == &BigUint::from(1u32) {
            c.clone()
        } else if d % 2u32 == BigUint::from(1u32) {
            self.add(c, &self.scalar_mul(c, &(d - 1u32)))
        } else {
            self.scalar_mul(&self.double(c), &(d / 2u32))
        }
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        //y^2 = x^3 + ax + b
        match c {
            Point::Coordinate(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, x, &self.p);
                let axb = FiniteField::add(&ax, &self.b, &self.p);
                y2 == FiniteField::add(&x3, &axb, &self.p)
            }
            Point::Identity => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_curve_0() {
        let ecc = EllipticCurve {
            a: BigUint::from(1u32),
            b: BigUint::from(2u32),
            p: BigUint::from(11u32),
        };
        let x = BigUint::from(1u32);
        let y = BigUint::from(2u32);
        assert_eq!(ecc.is_on_curve(&Point::Coordinate(x, y)), true);
    }

    #[test]
    fn test_ec_point_addition_0() {
        let ecc = EllipticCurve {
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
    }

    #[test]
    fn test_ec_point_addition_1() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);

        let res = ecc.add(&p1, &p2);
        assert_eq!(res, p1);
    }

    #[test]
    fn test_ec_point_addition_2() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);

        let res = ecc.add(&p2, &p1);
        assert_eq!(res, p1);
    }

    #[test]
    fn test_ec_point_addition_3() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        // (6,3) + (6,14) = inf
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(6u32), BigUint::from(14u32));
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&p2), true);

        let res = ecc.add(&p2, &p1);
        assert_eq!(res, Point::Identity);
    }

    #[test]
    fn test_ec_point_double_0() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        // (5,1) + (5,1) = (6,3)
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&r), true);

        let res = ecc.double(&p1);
        assert_eq!(res, r);
    }

    #[test]
    fn test_ec_point_double_1() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&r), true);

        let res = ecc.add(&p1, &p1);
        assert_eq!(res, r);
    }

    #[test]
    fn test_ec_point_double_2() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Identity;
        let r = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);
        assert_eq!(ecc.is_on_curve(&r), true);

        let res = ecc.double(&p1);
        assert_eq!(res, r);
    }

    #[test]
    fn test_ec_point_double_3() {
        let ecc = EllipticCurve {
            // y^2 = x^3 + 3x + 13 in F_17
            a: BigUint::from(3u32),
            b: BigUint::from(13u32),
            p: BigUint::from(17u32),
        };

        // Point (1, 0) is on the curve
        let p1 = Point::Coordinate(BigUint::from(1u32), BigUint::from(0u32));
        assert_eq!(ecc.is_on_curve(&p1), true);

        // Doubling a point with y = 0 should give Identity
        let res = ecc.double(&p1);
        assert_eq!(res, Point::Identity);
    }

    #[test]
    fn test_ec_scalar_mult_0() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let r1: Point = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let r2: Point = Point::Coordinate(BigUint::from(7u32), BigUint::from(11u32));
        let r3: Point = Point::Identity;
        let r4: Point = Point::Coordinate(BigUint::from(5u32), BigUint::from(16u32));
        assert_eq!(ecc.is_on_curve(&p1), true);

        let res = ecc.scalar_mul(&p1, &BigUint::from(2u32));
        assert_eq!(res, r1);

        let res = ecc.scalar_mul(&p1, &BigUint::from(10u32));
        assert_eq!(res, r2);

        let res = ecc.scalar_mul(&p1, &BigUint::from(19u32));
        assert_eq!(res, r3);

        let res = ecc.scalar_mul(&p1, &BigUint::from(18u32));
        assert_eq!(res, r4);
    }

    #[test]
    fn test_ec_scalar_mult_1() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Identity;
        let r = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);

        let res = ecc.scalar_mul(&p1, &BigUint::from(2u32));
        assert_eq!(res, r);
    }

    #[test]
    fn test_ec_scalar_mult_2() {
        let ecc = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Identity;
        assert_eq!(ecc.is_on_curve(&p1), true);

        let res = ecc.scalar_mul(&p1, &BigUint::from(0u32));
        assert_eq!(res, r);
    }

    #[test]
    fn test_ec_secp256k1() {
        let p: BigUint = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).expect("could not convert p");
        let n: BigUint = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).expect("could not convert n");
        let gx: BigUint = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).expect("could not convert gx");
        let gy: BigUint = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).expect("could not convert gy");
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };
        let g = Point::Coordinate(gx, gy);
        let res = ec.scalar_mul(&g, &n);
        assert_eq!(res, Point::Identity)
    }
}
