
/// 
/// Leetcode 207
/// Course Schedule
///
pub fn can_finish_207(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // [i][j] means ith course depends on jth course
    let mut course_graph_metrics: Vec<Vec<bool>> = vec![vec![false; num_courses as usize]; num_courses as usize];
    let mut row_index = 0;
    let prerequisites_len = prerequisites.len();
    while row_index < prerequisites_len {
        if prerequisites[row_index][0] == prerequisites[row_index][1] {
            return false;
        }
        course_graph_metrics[prerequisites[row_index][0] as usize][prerequisites[row_index][1] as usize] = true;
        row_index += 1;
    }
    /// row: current loop row
    /// all_visited: current_all_visited vec
    /// current_visited: current loop visited, if one exists, has loop dependency, return false
    /// graph_metrics: course_graph_metrics
    fn inner_course_schedule(row: usize, all_visited: &mut Vec<i32>, current_visited: &mut Vec<i32>, graph_metrics: &Vec<Vec<bool>>, num_courses: usize) -> bool {
        // current row has touched
        if all_visited[row] == 1 {
            return true;
        }
        if current_visited[row] == 1 {
            return false;
        }
        current_visited[row] = 1;
        let mut index = 0;
        let mut ret = true;
        while index < num_courses {
            if graph_metrics[row][index] { // current i j has dependency
                let next_index_ret = inner_course_schedule(index, all_visited, current_visited, graph_metrics, num_courses);
                if !next_index_ret {
                    ret = false;
                    break;
                }
            }
            index += 1;
        }
        // backtrack
        current_visited[row] = 0;
        // record all_visited
        all_visited[row] = 1;
        return ret;
    }
    let mut all_visited: Vec<i32> = vec![0; num_courses as usize];
    let mut current_visited: Vec<i32> = vec![0; num_courses as usize];
    let mut ret = true;
    row_index = 0;
    while row_index < num_courses as usize {
        let current_row_ret = inner_course_schedule(row_index, &mut all_visited, &mut current_visited, &course_graph_metrics, num_courses as usize);
        if !current_row_ret {
            ret = false;
            break;
        }
        row_index += 1;
    }
    ret
}
