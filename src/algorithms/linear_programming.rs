//! Linear programming problems. Ugly translation from C.

use crate::data_structures::array_2d::Array2D;

pub fn transportation_problem_vam(
    supply: &mut [i32],
    demand: &mut [i32],
    costs: &Array2D<i32>,
) -> (i32, Array2D<i32>) {
    let mut res = Array2D::new(0, demand.len(), supply.len());
    let mut rowf = vec![false; supply.len()];
    let mut colf = vec![false; demand.len()];
    let mut cost = 0;
    let mut total = supply.iter().sum::<i32>();

    while total > 0 {
        let (y, x) = calc(costs, &rowf, &colf);

        let min = if demand[x] < supply[y] {
            demand[x]
        } else {
            supply[y]
        };

        demand[x] -= min;
        if demand[x] == 0 {
            colf[x] = true;
        }

        supply[y] -= min;
        if supply[y] == 0 {
            rowf[y] = true;
        }

        res[y][x] = min;

        total -= min;

        cost += min * costs[y][x];
    }

    (cost, res)
}

fn calc(costs: &Array2D<i32>, rowf: &[bool], colf: &[bool]) -> (usize, usize) {
    let (ry, rx, rmin_cost, rmax_diff) = calc_pen(costs, rowf, colf, true);
    let (cx, cy, cmin_cost, cmax_diff) = calc_pen(costs, rowf, colf, false);

    if rmax_diff < cmax_diff {
        return (ry, rx);
    }

    if rmax_diff > cmax_diff {
        return (cy, cx);
    }

    if rmin_cost < cmin_cost {
        (ry, rx)
    } else {
        (cy, cx)
    }
}

fn calc_pen(
    costs: &Array2D<i32>,
    rowf: &[bool],
    colf: &[bool],
    is_row: bool,
) -> (usize, usize, i32, i32) {
    let rows = rowf.len();
    let cols = colf.len();
    let (len_main, len_aux) = if is_row { (rows, cols) } else { (cols, rows) };

    let mut minc_idx = 0;
    let mut maxd_idx = 0;
    let mut min_cost = -1;
    let mut max_diff = i32::MIN;

    for i in 0..len_main {
        let c = if is_row { rowf[i] } else { colf[i] };
        if c {
            continue;
        }

        let mut lmin_cost = i32::MAX;
        let mut lmin_cst2 = lmin_cost;
        let mut lminc_idx = 0;

        for k in 0..len_aux {
            let c = if is_row { colf[k] } else { rowf[k] };
            if c {
                continue;
            }

            let cost = if is_row { costs[i][k] } else { costs[k][i] };

            if cost < lmin_cost {
                lmin_cst2 = lmin_cost;
                lmin_cost = cost;
                lminc_idx = k;
            } else if cost < lmin_cst2 {
                lmin_cst2 = cost;
            }
        }

        let lmax_diff = lmin_cst2 - lmin_cost;

        if lmax_diff > max_diff {
            max_diff = lmax_diff;
            maxd_idx = i;
            min_cost = lmin_cost;
            minc_idx = lminc_idx;
        }
    }

    (maxd_idx, minc_idx, min_cost, max_diff)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut supply = [50, 60, 50, 50];
        let mut demand = [30, 20, 70, 30, 60];
        let costs = Array2D::from_slice(
            demand.len(),
            supply.len(),
            &[
                16, 16, 13, 22, 17,
                14, 14, 13, 19, 15,
                19, 19, 20, 23, 50,
                50, 12, 50, 15, 11,
            ]
        );

        let (cost, res) = transportation_problem_vam(&mut supply, &mut demand, &costs);

        assert_eq!(3100, cost);
        assert_eq!(
            &[
                 0,  0, 50,  0,  0,
                30,  0, 20,  0, 10,
                 0, 20,  0, 30,  0,
                 0,  0,  0,  0, 50,
            ],
            res.as_ref()
        );
    }

    #[test]
    fn case2() {
        let mut supply = [461, 277, 356, 488,  393];
        let mut demand = [278,  60, 461, 116, 1060];
        let costs = Array2D::from_slice(
            demand.len(),
            supply.len(),
            &[
                46,  74,  9, 28, 99,
                12,  75,  6, 36, 48,
                35, 199,  4,  5, 71,
                61,  81, 44, 88,  9,
                85,  60, 14, 25, 79,
            ]
        );

        let (cost, res) = transportation_problem_vam(&mut supply, &mut demand, &costs);

        assert_eq!(60748, cost);
        assert_eq!(
            &[
                   0,   0, 461,   0,   0,
                 277,   0,   0,   0,   0,
                   1,   0,   0,   0, 355,
                   0,   0,   0,   0, 488,
                   0,  60,   0, 116, 217,
            ],
            res.as_ref()
        );
    }
}
