use crate::*;
use rand::{thread_rng, Rng};

use rand::distributions::uniform::SampleUniform as SlU;
use std::cmp::Ord;
use std::cmp::PartialOrd as PlO;

pub fn rand_1_or_2<T>(a: T, b: T) -> T {
    return if rand_number([0, 1]) == 1 { a } else { b };
}

pub fn rand_bool() -> bool {
    return if rand_number([0, 1]) == 1 { true } else { false };
}

pub fn rand_number<T: Add<Output = T> + PlO + SlU + Ord + Clone>(mut number: [T; 2]) -> T {
    number.sort();

    return thread_rng().gen_range(number[0].clone()..=number[1].clone());
}
