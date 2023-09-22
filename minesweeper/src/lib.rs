use itertools::iproduct;

fn count_star_around(minefield: &Vec<Vec<char>>, row: usize, col: usize) ->  char {
    let dimension_row = minefield.len();
    let dimension_col = minefield[0].len();
    let mut count: u8 = 0;
    if row > 0 {
        if minefield[row - 1][col] == '*' {
            count += 1;
        }
        if col > 0 && minefield[row - 1][col - 1] == '*' {
            count += 1;
        }
        if col < dimension_col - 1 && minefield[row - 1][col + 1] == '*'{
            count += 1;
        }
    }
    if row < dimension_row - 1 {
        if minefield[row + 1][col] == '*' {
            count += 1;
        }
        if col > 0 && minefield[row + 1][col - 1] == '*'{
            count += 1;
        }
        if col < dimension_col - 1 && minefield[row + 1][col + 1] == '*'{
            count += 1;
        }
    }
    if col > 0 && minefield[row][col - 1] == '*'{
        count += 1;
    }
    if col < dimension_col - 1 && minefield[row][col + 1] == '*'{
        count += 1;
    }

    if count == 0 {
        ' '
    }else {
        ('0' as u8 + count) as char 
    }

}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let dimension_row = minefield.len();
    if dimension_row == 0{
        return vec![];
    }
    let dimension_col = minefield[0].len();

    let mut result: Vec<Vec<char>> = minefield.iter().map(|x| x.chars().collect()).collect();

    for (x, y) in iproduct!(0..dimension_row, 0..dimension_col) {
        if result[x][y] == '*' {
            continue;
        }
        result[x][y] = count_star_around(&result, x, y);
    }

    result
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect()
}
