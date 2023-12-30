use crate::*;

// AABB

#[derive(Debug, Clone, Copy)]
pub struct AabbS {
    pub(crate) position: PositionS,
    pub(crate) size: SizeS,
}

pub type FactsAabb = (FactsPosition, FactsSize);

impl Create for AabbS {
    type Output = AabbS;
    type Facts = FactsAabb;

    fn new(facts: &Self::Facts) -> Self::Output {
        AabbS {
            position: PositionS::new(&facts.0),
            size: SizeS::new(&facts.1),
        }
    }

    fn default() -> Self::Output {
        AabbS {
            position: PositionS::default(),
            size: SizeS::default(),
        }
    }
}

impl AabbS {
    pub fn aabb_add_aabb(aabb_0: &AabbS, aabb_1: &AabbS) -> AabbS {
        AabbS {
            position: PositionS::new(&[
                aabb_0.position.x + aabb_1.position.x,
                aabb_0.position.y + aabb_1.position.y,
            ]),
            size: SizeS::new(&[aabb_1.size.width, aabb_1.size.height]),
        }
    }
}

// Coordinates

#[derive(Debug, Clone, Copy)]
pub struct PositionS {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

pub type FactsPosition = [u16; 2];

impl Create for PositionS {
    type Output = PositionS;
    type Facts = FactsPosition;

    fn new(facts: &Self::Facts) -> Self::Output {
        PositionS {
            x: facts[0],
            y: facts[1],
        }
    }

    fn default() -> Self::Output {
        PositionS { x: 0, y: 0 }
    }
}

impl Geometry for PositionS {
    type Output = PositionS;

    fn add(geometry: &mut Self::Output, data: &[Result<u16, u16>; 2]) {
        match data[0] {
            Ok(dat) => geometry.x += dat,
            Err(dat) => {
                if geometry.x - dat > 0 {
                    geometry.x -= dat;
                }
            }
        }

        match data[1] {
            Ok(dat) => geometry.y += dat,
            Err(dat) => {
                if geometry.y - dat > 0 {
                    geometry.y -= dat;
                }
            }
        }
    }

    fn change(geometry: &mut Self::Output, data: &[u16; 2]) {
        geometry.x = data[0];
        geometry.y = data[1];
    }
}

// Size

#[derive(Debug, Clone, Copy)]
pub struct SizeS {
    pub(crate) width: u16,
    pub(crate) height: u16,
}

pub type FactsSize = [u16; 2];

impl Create for SizeS {
    type Output = SizeS;
    type Facts = FactsSize;

    fn new(facts: &Self::Facts) -> Self::Output {
        SizeS {
            width: facts[0],
            height: facts[1],
        }
    }

    fn default() -> Self::Output {
        SizeS {
            width: 0,
            height: 0,
        }
    }
}

impl Geometry for SizeS {
    type Output = SizeS;

    fn add(geometry: &mut Self::Output, data: &[Result<u16, u16>; 2]) {
        match data[0] {
            Ok(dat) => geometry.width += dat,
            Err(dat) => {
                if geometry.width - dat > 0 {
                    geometry.width -= dat;
                }
            }
        }

        match data[1] {
            Ok(dat) => geometry.height += dat,
            Err(dat) => {
                if geometry.height - dat > 0 {
                    geometry.height -= dat;
                }
            }
        }
    }

    fn change(geometry: &mut Self::Output, data: &[u16; 2]) {
        geometry.width = data[0];
        geometry.height = data[1];
    }
}
