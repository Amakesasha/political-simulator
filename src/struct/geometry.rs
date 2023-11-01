pub mod aabb {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct AabbS {
        pub(crate) position: PositionS,
        pub(crate) size: Scale,
    }

    pub type FactsAabb = (FactsPosition, FactsScale);

    impl Create for AabbS {
        type Output = AabbS;
        type Facts = FactsAabb;

        fn new(facts: Self::Facts) -> Self::Output {
            AabbS {
                position: PositionS::new(facts.0),
                size: Scale::new(facts.1),
            }
        }

        fn default() -> Self::Output {
            AabbS {
                position: PositionS::default(),
                size: Scale::default(),
            }
        }
    }

    impl AabbS {
        pub fn aabb_add_aabb(aabb_0: &AabbS, aabb_1: &AabbS) -> AabbS {
            AabbS {
                position: PositionS::new([
                    aabb_0.position.x + aabb_1.position.x,
                    aabb_0.position.y + aabb_1.position.y,
                ]),
                size: Scale::new([
                    aabb_0.position.x + aabb_1.position.x + aabb_1.size.width,
                    aabb_0.position.y + aabb_1.position.y + aabb_1.size.height,
                ]),
            }
        }
    }
}

pub mod position {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct PositionS {
        pub(crate) x: f64,
        pub(crate) y: f64,
    }

    pub type FactsPosition = [f64; 2];

    impl Create for PositionS {
        type Output = PositionS;
        type Facts = FactsPosition;

        fn new(facts: Self::Facts) -> Self::Output {
            PositionS {
                x: facts[0],
                y: facts[1],
            }
        }

        fn default() -> Self::Output {
            PositionS { x: 0.0, y: 0.0 }
        }
    }
}

pub mod size {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct Scale {
        pub(crate) width: f64,
        pub(crate) height: f64,
    }

    pub type FactsScale = [f64; 2];

    impl Create for Scale {
        type Output = Scale;
        type Facts = FactsScale;

        fn new(facts: Self::Facts) -> Self::Output {
            Scale {
                width: facts[0],
                height: facts[1],
            }
        }

        fn default() -> Self::Output {
            Scale {
                width: 0.0,
                height: 0.0,
            }
        }
    }
}
