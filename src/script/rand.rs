use rand::Rng;

pub fn rand_isize(number_0: isize, number_1: isize) -> isize {
    return if number_0 > number_1 {
        rand::thread_rng().gen_range(number_0..=number_1)
    } else if number_1 > number_0 {
        rand::thread_rng().gen_range(number_1..=number_0)
    } else {
        number_0
    };
}

pub fn rand_usize(number_0: usize, number_1: usize) -> usize {
    return if number_0 > number_1 {
        rand::thread_rng().gen_range(number_0..=number_1)
    } else if number_1 > number_0 {
        rand::thread_rng().gen_range(number_1..=number_0)
    } else {
        number_0
    };
}

pub fn rand_f64(number_0: f64, number_1: f64) -> f64 {
    return if number_0 > number_1 {
        rand::thread_rng().gen_range(number_0..=number_1)
    } else if number_1 > number_0 {
        rand::thread_rng().gen_range(number_1..=number_0)
    } else {
        number_0
    };
}

pub fn rand_f32(number_0: f32, number_1: f32) -> f32 {
    return if number_0 > number_1 {
        rand::thread_rng().gen_range(number_0..=number_1)
    } else if number_1 > number_0 {
        rand::thread_rng().gen_range(number_1..=number_0)
    } else {
        number_0
    };
}
