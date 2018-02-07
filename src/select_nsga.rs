use selection::SelectAndRank;
use std::cmp::PartialOrd;
use non_dominated_sort::NonDominatedSort;
use multi_objective::MultiObjective;
use crowding_distance::{assign_crowding_distance, AssignedCrowdingDistance};

/// Select `n` solutions using the approach taken by NSGA.
///
/// We first sort the solutions into their corresponding pareto fronts
/// using a non-dominated sort algorithm. Then, we put as many
/// "complete" fronts into the result set, until we cannot fit in a
/// whole front anymore, without exceeding `n` solutions in the result
/// set. For this last front, that does not completely fit into the
/// result set, we sort it's solutions according to their crowding
/// distance (higher crowding distance is "better"), and prefer those
/// solutions with the higher crowding distance until we have exactly
/// `n` solutions in the result set.

pub struct SelectNSGA;

impl SelectAndRank for SelectNSGA {
    fn select_and_rank<'a, S: 'a>(
        &self,
        solutions: &'a [S],
        n: usize,
        multi_objective: &MultiObjective<S, f64>,
    ) -> Vec<AssignedCrowdingDistance<'a, S>> {
        // Cannot select more solutions than we actually have
        let n = solutions.len().min(n);
        debug_assert!(n <= solutions.len());

        let mut result = Vec::with_capacity(n);
        let mut missing_solutions = n;

        for front in NonDominatedSort::new(solutions, multi_objective) {
            let (mut assigned_crowding, _) = assign_crowding_distance(&front, multi_objective);

            if assigned_crowding.len() > missing_solutions {
                // the front does not fit in total. sort it's solutions
                // according to the crowding distance and take the best
                // solutions until we have "n" solutions in the result.

                assigned_crowding.sort_by(|a, b| {
                    debug_assert_eq!(a.rank, b.rank);
                    a.crowding_distance
                        .partial_cmp(&b.crowding_distance)
                        .unwrap()
                        .reverse()
                });
            }

            // Take no more than `missing_solutions`
            let take = assigned_crowding.len().min(missing_solutions);

            result.extend(assigned_crowding.into_iter().take(take));

            missing_solutions -= take;
            if missing_solutions == 0 {
                break;
            }
        }

        debug_assert_eq!(n, result.len());

        return result;
    }
}
