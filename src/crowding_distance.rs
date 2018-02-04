use multi_objective::MultiObjective;
use non_dominated_sort::Front;
use std::f64::INFINITY;

pub struct AssignedCrowdingDistance<'a, S>
where
    S: 'a,
{
    index: usize,
    solution: &'a S,
    rank: usize,
    crowding_distance: f64,
}

pub struct ObjectiveStat {
    spread: f64,
}

/// Assigns a crowding distance to each solution in `front`.
pub fn assign_crowding_distance<'a, S>(
    front: Front<'a, S>,
    multi_objective: MultiObjective<S, f64>,
) -> (Vec<AssignedCrowdingDistance<'a, S>>, Vec<ObjectiveStat>) {
    let mut a: Vec<_> = front
        .solutions
        .iter()
        .map(|i| AssignedCrowdingDistance {
            index: i.index,
            solution: i.solution,
            rank: front.rank,
            crowding_distance: 0.0,
        })
        .collect();

    let objective_stat: Vec<_> = multi_objective
        .objectives
        .iter()
        .map(|objective| {
            // First, sort according to objective
            a.sort_by(|a, b| objective.total_order(a.solution, b.solution));

            // Assign infinite crowding distance to the extremes
            {
                a.first_mut().unwrap().crowding_distance = INFINITY;
                a.last_mut().unwrap().crowding_distance = INFINITY;
            }

            // The distance between the "best" and "worst" solution
            // according to "objective".
            let spread = objective
                .distance(a.first().unwrap().solution, a.last().unwrap().solution)
                .abs();
            debug_assert!(spread >= 0.0);

            if spread > 0.0 {
                let norm = 1.0 / (spread * (multi_objective.objectives.len() as f64));
                debug_assert!(norm > 0.0);

                for i in 1..a.len() - 1 {
                    debug_assert!(i >= 1 && i + 1 < a.len());

                    let distance = objective
                        .distance(a[i + 1].solution, a[i - 1].solution)
                        .abs();
                    debug_assert!(distance >= 0.0);
                    a[i].crowding_distance += distance * norm;
                }
            }

            ObjectiveStat { spread }
        })
        .collect();

    return (a, objective_stat);
}
