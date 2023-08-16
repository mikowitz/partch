use crate::ratio::Ratio;

#[derive(Clone, Copy, Debug)]
pub enum DimensionBound {
    Infinity,
    ZeroBounded(i32),
    RangeBounded(i32, i32),
}

#[derive(Copy, Clone, Debug)]
pub struct LatticeDimension {
    pub ratio: Ratio,
    pub bounds: DimensionBound,
}

impl LatticeDimension {
    pub fn resolve_index(&self, index: i32) -> i32 {
        match self.bounds {
            DimensionBound::Infinity => index,
            DimensionBound::ZeroBounded(n) => sign_preserving_mod(index, n),
            DimensionBound::RangeBounded(a, b) => {
                let modulo = b - a + 1;
                let abs_a = a.abs();

                sign_preserving_mod(index + abs_a, modulo) - abs_a
            }
        }
    }
}

fn sign_preserving_mod(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

#[derive(Debug)]
pub struct Lattice {
    pub dimensions: Vec<LatticeDimension>,
}

impl Lattice {
    pub fn new(dimensions: Vec<LatticeDimension>) -> Self {
        Self { dimensions }
    }

    pub fn at(&self, indices: Vec<i32>) -> Ratio {
        self.dimensions
            .iter()
            .zip(indices.iter())
            .map(|(&dim, &index)| dim.ratio.pow(dim.resolve_index(index)))
            .fold(Ratio::new(1, 1), |e, acc| acc * e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DimensionBound::*;

    #[test]
    fn resolve_unbounded_dimension() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: Infinity,
        };

        assert_eq!(dim.resolve_index(0), 0);
        assert_eq!(dim.resolve_index(3), 3);
        assert_eq!(dim.resolve_index(-2), -2);
    }

    #[test]
    fn resolve_zero_bounded_dimension() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: ZeroBounded(2),
        };

        assert_eq!(dim.resolve_index(0), 0);
        assert_eq!(dim.resolve_index(1), 1);
        assert_eq!(dim.resolve_index(3), 1);
        assert_eq!(dim.resolve_index(-1), 1);
        assert_eq!(dim.resolve_index(-2), 0);
    }

    #[test]
    fn resolve_range_bounded_dimension() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: RangeBounded(-1, 2),
        };

        assert_eq!(dim.resolve_index(0), 0);
        assert_eq!(dim.resolve_index(1), 1);
        assert_eq!(dim.resolve_index(2), 2);
        assert_eq!(dim.resolve_index(3), -1);
        assert_eq!(dim.resolve_index(-2), 2);
    }

    #[test]
    fn resolve_zero_bounded_negative_dimension() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: ZeroBounded(-2),
        };

        assert_eq!(dim.resolve_index(0), 0);
        assert_eq!(dim.resolve_index(1), -1);
        assert_eq!(dim.resolve_index(3), -1);
        assert_eq!(dim.resolve_index(-2), 0);
    }

    #[test]
    fn one_dimensional_unbounded_lattice() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: Infinity,
        };

        let lattice = Lattice::new(vec![dim]);

        assert_eq!(lattice.at(vec![0]), Ratio::new(1, 1));
        assert_eq!(lattice.at(vec![1]), Ratio::new(3, 2));
        assert_eq!(lattice.at(vec![2]), Ratio::new(9, 4));
        assert_eq!(lattice.at(vec![-1]), Ratio::new(4, 3));
        assert_eq!(lattice.at(vec![-2]), Ratio::new(16, 9));
    }

    #[test]
    fn one_dimensional_zero_bounded_lattice() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: ZeroBounded(2),
        };

        let lattice = Lattice::new(vec![dim]);

        assert_eq!(lattice.at(vec![0]), Ratio::new(1, 1));
        assert_eq!(lattice.at(vec![1]), Ratio::new(3, 2));
        assert_eq!(lattice.at(vec![2]), Ratio::new(1, 1));
        assert_eq!(lattice.at(vec![-1]), Ratio::new(3, 2));
        assert_eq!(lattice.at(vec![-2]), Ratio::new(1, 1));
    }

    #[test]
    fn one_dimensional_range_bounded_lattice() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: RangeBounded(-2, 3),
        };

        let lattice = Lattice::new(vec![dim]);

        assert_eq!(lattice.at(vec![0]), Ratio::new(1, 1));
        assert_eq!(lattice.at(vec![1]), Ratio::new(3, 2));
        assert_eq!(lattice.at(vec![2]), Ratio::new(9, 4));
        assert_eq!(lattice.at(vec![3]), Ratio::new(27, 8));
        assert_eq!(lattice.at(vec![4]), Ratio::new(16, 9));
        assert_eq!(lattice.at(vec![-1]), Ratio::new(4, 3));
        assert_eq!(lattice.at(vec![-2]), Ratio::new(16, 9));
        assert_eq!(lattice.at(vec![-3]), Ratio::new(27, 8));
    }

    #[test]
    fn range_bound_with_zero_is_different_than_zero_bound() {
        let dim = LatticeDimension {
            ratio: Ratio::new(3, 2),
            bounds: RangeBounded(0, 2),
        };

        let lattice = Lattice::new(vec![dim]);

        assert_eq!(lattice.at(vec![0]), Ratio::new(1, 1));
        assert_eq!(lattice.at(vec![1]), Ratio::new(3, 2));
        assert_eq!(lattice.at(vec![2]), Ratio::new(9, 4));
        assert_eq!(lattice.at(vec![3]), Ratio::new(1, 1));
    }

    #[test]
    fn two_dimensional_unbounded_lattice() {
        let lattice = Lattice::new(vec![
            LatticeDimension {
                ratio: Ratio::new(3, 2),
                bounds: Infinity,
            },
            LatticeDimension {
                ratio: Ratio::new(5, 4),
                bounds: Infinity,
            },
        ]);

        assert_eq!(lattice.at(vec![1, 1]), Ratio::new(15, 8))
    }
}
