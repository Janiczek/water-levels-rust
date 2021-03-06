#[macro_use]
extern crate static_assertions;

use std::collections::VecDeque;
use std::f64::INFINITY;
use math::round;

const ITER_PRECISION: i8 = 5;
const OUTPUT_PRECISION: i8 = 4;
const_assert!(OUTPUT_PRECISION < ITER_PRECISION);

#[allow(clippy::float_cmp)]
fn are_same(a: &[f64], b: &[f64]) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn rounded(a: &[f64], precision: i8) -> Vec<f64> {
    a.iter().map(|x| round::half_up(*x, precision)).collect()
}

pub fn rain(landscape: &[i32], hours: i32) -> Vec<f64> {
    let fhours: f64 = hours as f64;

    let mut columns_deque: VecDeque<f64> = landscape.iter().map(|x| *x as f64).collect();
    columns_deque.push_front(INFINITY);
    columns_deque.push_back(INFINITY);

    let columns: Vec<f64> = columns_deque.into_iter().collect();
    let levels: Vec<f64> = columns.iter().map(|x| x + fhours).collect();

    stabilize(&columns, &levels)
}

fn stabilize(columns: &[f64], levels: &[f64]) -> Vec<f64> {
    let mut has_changed = true;
    let mut old_levels: Vec<f64>;
    let mut new_levels: Vec<f64> = levels.to_vec();
    let length: usize = levels.len();
    while has_changed {
        old_levels = new_levels.clone();
        for i in 1..length - 1 {
            let column_center: f64 = columns[i];

            let level_left: f64 = old_levels[i - 1];
            let level_center: f64 = old_levels[i];
            let level_right: f64 = old_levels[i + 1];

            let flow_left: bool = level_left < level_center;
            let flow_right: bool = level_right < level_center;

            let current_water: f64 = level_center - column_center;
            let has_water_to_move: bool = current_water > 0.;

            if has_water_to_move {
                if flow_left && flow_right {
                    let moved_left: f64 = (current_water / 2.).min((level_center - level_left) / 2.);
                    let moved_right: f64 = (current_water / 2.).min((level_center - level_right) / 2.);
                    new_levels[i - 1] += moved_left;
                    new_levels[i] -= moved_left + moved_right;
                    new_levels[i + 1] += moved_right;
                } else if flow_left {
                    let moved_left: f64 = current_water.min((level_center - level_left) / 2.);
                    new_levels[i - 1] += moved_left;
                    new_levels[i] -= moved_left;
                } else if flow_right {
                    let moved_right: f64 = current_water.min((level_center - level_right) / 2.);
                    new_levels[i] -= moved_right;
                    new_levels[i + 1] += moved_right;
                }
            }
        }
        has_changed = !are_same(&rounded(&old_levels, ITER_PRECISION),
                                &rounded(&new_levels, ITER_PRECISION));
    }
    rounded(&new_levels[1..levels.len() - 1].to_vec(), OUTPUT_PRECISION)
}
