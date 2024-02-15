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
}
