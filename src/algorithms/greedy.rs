#[derive(Clone, Copy)]
pub struct Activity {
    start: u32,
    end: u32,
}

pub fn activity_selection(activities: &[Activity]) -> Vec<Activity> {
    let mut activities = activities.to_vec();

    activities.sort_by(|a, b| a.end.cmp(&b.end));

    let mut answer = vec![activities[0]];
    let mut k = 0;

    for i in 1..activities.len() {
        if activities[i].start >= activities[k].end {
            answer.push(activities[i]);
            k = i;
        }
    }

    answer
}

#[derive(Clone, Copy)]
pub struct Item {
    value: u32,
    weight: u32,
}

// casts are u32 -> f64 lesser than original, and back to u32
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn fractional_knapsack(items: &[Item], max_weight: u32) -> Vec<Item> {
    let mut sorted = items.to_vec();

    sorted.sort_by(|a, b| {
        let fa = f64::from(a.value) / f64::from(a.weight);
        let fb = f64::from(b.value) / f64::from(b.weight);

        fb.partial_cmp(&fa)
            .expect("input contains zero-weight item")
    });

    let items = sorted;

    let mut answer = vec![];
    let mut total_weight = 0;
    let mut i = 0;

    while total_weight < max_weight && i < items.len() {
        let remaining_weight = max_weight - total_weight;

        if items[i].weight <= remaining_weight {
            answer.push(items[i]);
            total_weight += items[i].weight;
        } else {
            let frac = f64::from(remaining_weight) / f64::from(items[i].weight);

            let frac_value = (f64::from(items[i].value) * frac).round() as u32;
            let frac_weight = (f64::from(items[i].weight) * frac).round() as u32;

            answer.push(Item {
                value: frac_value,
                weight: frac_weight,
            });

            total_weight += frac_weight;
        }

        i += 1;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_selection_test() {
        let activities = [
            (1, 4),
            (3, 5),
            (0, 6),
            (5, 7),
            (3, 9),
            (5, 9),
            (6, 10),
            (8, 11),
            (8, 12),
            (2, 14),
            (12, 16),
        ]
        .map(|(start, end)| Activity { start, end });

        let ans = activity_selection(&activities);

        assert_eq!(4, ans.len());
    }

    #[test]
    fn fractional_knapsack_test() {
        let items = [
            Item {
                value: 60,
                weight: 10,
            },
            Item {
                value: 100,
                weight: 20,
            },
            Item {
                value: 120,
                weight: 30,
            },
        ];
        let max_weight = 50;
        let ans = fractional_knapsack(&items, max_weight);
        let total_value = ans.iter().fold(0, |acc, x| acc + x.value);

        assert_eq!(240, total_value);
    }
}
