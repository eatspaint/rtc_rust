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

    /// Adds two Tuples together, returning sum Tuple
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a1 = Tuple { x: 3., y: -2., z: 5., w: 1. };
    /// let a2 = Tuple { x: -2., y: 3., z: 1., w: 0. };
    /// let expected = Tuple { x: 1., y: 1., z: 6., w: 1. };
    /// let actual = a1 + a2;
    /// assert_eq!(actual, expected);
    /// ```
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

    /// Subtracts one Tuple from another, returning difference Tuple
    ///
    /// # Examples
    /// 
    /// Subtracting a point from a point
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a1 = Tuple::point(3., 2., 1.);
    /// let a2 = Tuple::point(5., 6., 7.);
    /// let expected = Tuple::vector(-2., -4., -6.);
    /// let actual = a1 - a2;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// Subtracting a vector from a point
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let p = Tuple::point(3., 2., 1.);
    /// let v = Tuple::vector(5., 6., 7.);
    /// let expected = Tuple::point(-2., -4., -6.);
    /// let actual = p - v;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// Subtracting a vector from a vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v1 = Tuple::vector(3., 2., 1.);
    /// let v2 = Tuple::vector(5., 6., 7.);
    /// let expected = Tuple::vector(-2., -4., -6.);
    /// let actual = v1 - v2;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// Subtracting a vector from a zero vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(1., -2., 3.);
    /// let zero = Tuple::vector(0., 0., 0.);
    /// let expected = Tuple::vector(-1., 2., -3.);
    /// let actual = zero - v;
    /// assert_eq!(actual, expected);
    /// ```

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

    /// Returns negated Tuple
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
    /// let expected = Tuple { x: -1., y: 2., z: -3., w: 4.};
    /// let actual = -a;
    /// assert_eq!(actual, expected);
    /// ```
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

    /// Multiplies a Tuple by a float
    ///
    /// # Examples
    ///
    /// Multiply by scalar
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
    /// let expected = Tuple { x: 3.5, y: -7., z: 10.5, w: -14.};
    /// let actual = a * 3.5;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// Multiply by fraction
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
    /// let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2.};
    /// let actual = a * 0.5;
    /// assert_eq!(actual, expected);
    /// ```
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

    /// Divides a Tuple by a float
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple { x: 1., y: -2., z: 3., w: -4.};
    /// let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2.};
    /// let actual = a / 2.;
    /// assert_eq!(actual, expected);
    /// ```
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
    /// Creates a new Tuple
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let tup = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
    /// assert_eq!(tup.x, 4.3);
    /// assert_eq!(tup.y, -4.2);
    /// assert_eq!(tup.z, 3.1);
    /// assert_eq!(tup.w, 1.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }

    /// Return bool indicating whether Tuple is a point
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let point = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
    /// assert_eq!(point.is_point(), true);
    /// ```
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Return bool indicating whether Tuple is a vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let vector = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
    /// assert_eq!(vector.is_vector(), true);
    /// ```
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Creates a new point Tuple
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let point = Tuple::point(4., -4., 3.);
    /// let tuple = Tuple { x: 4., y: -4., z: 3., w: 1. };
    /// assert_eq!(point.is_point(), true);
    /// assert_eq!(point, tuple);
    /// ```  
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 1.0}
    }

    /// Creates a new vector Tuple
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let vector = Tuple::vector(4., -4., 3.);
    /// let tuple = Tuple { x: 4., y: -4., z: 3., w: 0.};
    /// assert_eq!(vector.is_vector(), true);
    /// assert_eq!(vector, tuple);
    /// ```
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 0.0}
    }

    /// Returns magnitude of Tuple as float
    ///
    /// # Examples
    ///
    /// Magnitude of a unit X vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(1., 0., 0.);
    /// assert_eq!(v.mag(), 1.);
    /// ```
    ///
    /// Magnitude of a unit Y vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(0., 1., 0.);
    /// assert_eq!(v.mag(), 1.);
    /// ```
    ///
    /// Magnitude of a unit Z vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(0., 0., 1.);
    /// assert_eq!(v.mag(), 1.);
    /// ```
    ///
    /// Magnitude of non-unit vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(1., 2., 3.);
    /// assert_eq!(v.mag(), 14.0_f64.sqrt());
    /// ```
    ///
    /// Magnitude of negtated non-unit vector
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(-1., -2., -3.);
    /// assert_eq!(v.mag(), 14.0_f64.sqrt());
    /// ```
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) +
        self.y.powi(2) +
        self.z.powi(2) +
        self.w.powi(2)).sqrt()
    }
    
    /// Returns unit vector from given Tuple
    /// 
    /// # Examples
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(4., 0., 0.,);
    /// let expected = Tuple::vector(1., 0., 0.,);
    /// assert_eq!(v.norm(), expected);
    /// ```
    /// 
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(1., 2., 3.,);
    /// let expected = Tuple::vector(
    ///     1. / 14.0_f64.sqrt(),
    ///     2. / 14.0_f64.sqrt(),
    ///     3. / 14.0_f64.sqrt()
    /// );
    /// assert_eq!(v.norm(), expected);
    /// ```
    /// 
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let v = Tuple::vector(1., 2., 3.);
    /// let norm = v.norm();
    /// assert_eq!(norm.mag(), 1.);
    /// ```
    
    pub fn norm(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
    
    /// Returns dot product of two Tuples
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple::vector(1., 2., 3.);
    /// let b = Tuple::vector(2., 3., 4.);
    /// assert_eq!(a.dot(&b), 20.);
    /// ```
    pub fn dot(&self, cmp: &Self) -> f64 {
        (self.x * cmp.x) +
        (self.y * cmp.y) +
        (self.z * cmp.z) +
        (self.w * cmp.w)
    }
    
    /// Returns cross product of two Tuples
    ///
    /// ```
    /// # use rtc_rust::tuple::Tuple;
    /// let a = Tuple::vector(1., 2., 3.);
    /// let b = Tuple::vector(2., 3., 4.);
    /// let expected_a_b = Tuple::vector(-1., 2., -1.);
    /// let expected_b_a = Tuple::vector(1., -2., 1.);
    /// assert_eq!(a.cross(&b), expected_a_b);
    /// assert_eq!(b.cross(&a), expected_b_a);
    /// ```
    pub fn cross(&self, cmp: &Self) -> Self {
        Self::vector(
            self.y * cmp.z - self.z * cmp.y,
            self.z * cmp.x - self.x * cmp.z,
            self.x * cmp.y - self.y * cmp.x,
        )
    }
}
