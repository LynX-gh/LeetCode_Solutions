impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut x_arr = vec![];
        let mut y_arr = vec![];

        for rect in rectangles {
            x_arr.push((rect[0], rect[2]));
            y_arr.push((rect[1], rect[3]));
        }

        x_arr.sort_unstable_by_key(|rect| rect.0);
        let mut start = x_arr[0].0;
        let mut end = x_arr[0].1;
        let mut cuts = 0;

        for &(cur_start, cur_end) in x_arr.iter().skip(1) {
            if cur_start < end {
                end = cur_end.max(end);
                continue;
            }

            cuts += 1;
            if cuts == 2 {
                return true;
            }
            end = cur_end;
            start = cur_start;
        }

        y_arr.sort_unstable_by_key(|rect| rect.0);
        start = y_arr[0].0;
        end = y_arr[0].1;
        cuts = 0;

        for &(cur_start, cur_end) in y_arr.iter().skip(1) {
            if cur_start < end {
                end = cur_end.max(end);
                continue;
            }

            cuts += 1;
            if cuts == 2 {
                return true;
            }
            end = cur_end;
            start = cur_start;
        }
        false
    }
}