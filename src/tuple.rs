#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, cmp: & Self) -> bool {
        super::approx_eq(self.x, cmp.x) &&
        super::approx_eq(self.y, cmp.y) &&
        super::approx_eq(self.z, cmp.z) &&
        self.w == cmp.w
    }
}

impl std::ops::Add<Tuple> for Tuple {
    type Output = Self;

    fn add(self, cmp: Self) -> Self {
        Self { 
            x: self.x + cmp.x,
            y: self.y + cmp.y,
            z: self.z + cmp.z,
            w: self.w + cmp.w,
        }
    }
}

impl std::ops::Sub<Tuple> for Tuple {
    type Output = Self;

    fn sub(self, cmp: Self) -> Self {
        Self {
            x: self.x - cmp.x,
            y: self.y - cmp.y,
            z: self.z - cmp.z,
            w: self.w - cmp.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, cmp: f64) -> Self {
        Self {
            x: self.x * cmp,
            y: self.y * cmp,
            z: self.z * cmp,
            w: self.w * cmp,
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, cmp: f64) -> Self {
        Self {
            x: self.x / cmp,
            y: self.y / cmp,
            z: self.z / cmp,
            w: self.w / cmp,
        }
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 1.0}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 0.0}
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) +
         self.y.powi(2) +
         self.z.powi(2) +
         self.w.powi(2)).sqrt()
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, cmp: &Self) -> f64 {
        (self.x * cmp.x) +
        (self.y * cmp.y) +
        (self.z * cmp.z) +
        (self.w * cmp.w)
    }

    pub fn cross(&self, cmp: &Self) -> Self {
        Self::vector(
            self.y * cmp.z - self.z * cmp.y,
            self.z * cmp.x - self.x * cmp.z,
            self.x * cmp.y - self.y * cmp.x,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let point = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert_eq!(point.is_point(), true);
        assert_eq!(point.is_vector(), false);
    }

    #[test]
    fn tuple_is_vector() {
        let vector = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
        assert_eq!(vector.is_point(), false);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn point_creates_correct_tuple() {
        let point = Tuple::point(4., -4., 3.);
        let tuple = Tuple { x: 4., y: -4., z: 3., w: 1. };
        assert_eq!(point.is_point(), true);
        assert_eq!(point, tuple);
    }

    #[test]
    fn vector_creates_correct_tuple() {
        let vector = Tuple::vector(4., -4., 3.);
        let tuple = Tuple { x: 4., y: -4., z: 3., w: 0.};
        assert_eq!(vector.is_vector(), true);
        assert_eq!(vector, tuple);
    }

    #[test]
    fn addition_works() {
        let a1 = Tuple { x: 3., y: -2., z: 5., w: 1. };
        let a2 = Tuple { x: -2., y: 3., z: 1., w: 0. };
        let expected = Tuple { x: 1., y: 1., z: 6., w: 1. };
        let actual = a1 + a2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtracting_points_works() {
        let a1 = Tuple::point(3., 2., 1.);
        let a2 = Tuple::point(5., 6., 7.);
        let expected = Tuple::vector(-2., -4., -6.);
        let actual = a1 - a2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtracting_vector_from_point_works() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        let expected = Tuple::point(-2., -4., -6.);
        let actual = p - v;
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        let expected = Tuple::vector(-2., -4., -6.);
        let actual = v1 - v2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let v = Tuple::vector(1., -2., 3.);
        let zero = Tuple::vector(0., 0., 0.);
        let expected = Tuple::vector(-1., 2., -3.);
        let actual = zero - v;
        assert_eq!(actual, expected);
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
        let expected = Tuple { x: -1., y: 2., z: -3., w: 4.};
        let actual = -a;
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
        let expected = Tuple { x: 3.5, y: -7., z: 10.5, w: -14.};
        let actual = a * 3.5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_tuple_by_fraction() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2.};
        let actual = a * 0.5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2.};
        let actual = a / 2.;
        assert_eq!(actual, expected);
    }

    #[test]
    fn compute_magnitude_unit_x() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(v.mag(), 1.);
    }

    #[test]
    fn compute_magnitude_unit_y() {
        let v = Tuple::vector(0., 1., 0.);
        assert_eq!(v.mag(), 1.);
    }

    #[test]
    fn compute_magnitude_unit_z() {
        let v = Tuple::vector(0., 0., 1.);
        assert_eq!(v.mag(), 1.);
    }

    #[test]
    fn compute_magnitude_of_vector() {
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.mag(), 14.0_f64.sqrt());
    }

    #[test]
    fn compute_magnitude_of_neg_vector() {
        let v = Tuple::vector(-1., -2., -3.);
        assert_eq!(v.mag(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_1() {
        let v = Tuple::vector(4., 0., 0.,);
        let expected = Tuple::vector(1., 0., 0.,);
        assert_eq!(v.norm(), expected);
    }

    #[test]
    fn normalizing_vector_2() {
        let v = Tuple::vector(1., 2., 3.,);
        let expected = Tuple::vector(
            1. / 14.0_f64.sqrt(),
            2. / 14.0_f64.sqrt(),
            3. / 14.0_f64.sqrt()
        );
        assert_eq!(v.norm(), expected);
    }

    #[test]
    fn mag_of_norm_vector() {
        let v = Tuple::vector(1., 2., 3.);
        let norm = v.norm();
        assert_eq!(norm.mag(), 1.);
    }

    #[test]
    fn dot_product_of_tuples() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.dot(&b), 20.);
    }

    #[test]
    fn cross_product_of_vectors() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        let expected_a_b = Tuple::vector(-1., 2., -1.);
        let expected_b_a = Tuple::vector(1., -2., 1.);
        assert_eq!(a.cross(&b), expected_a_b);
        assert_eq!(b.cross(&a), expected_b_a);
    }
}
