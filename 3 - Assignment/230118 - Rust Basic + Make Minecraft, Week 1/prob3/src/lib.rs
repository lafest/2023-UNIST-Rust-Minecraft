pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(row_index, row)| {
        row.chars().enumerate().map(|(col_index, _)| {
            mine_num_to_annotation(calculate(minefield, (row_index, col_index)))
        }).collect::<Vec<String>>().join("")
    }).collect::<Vec<String>>()
}

fn mine_num_to_annotation(mine_no: i32) -> String {
    match mine_no {
        -1 => return '*'.to_string(),
        0 => return ' '.to_string(),
        mathched_num => return mathched_num.to_string(),
    }
}

fn calculate(minefield: &[&str], (row, col): (usize, usize)) -> i32 {
    if minefield[row].chars().nth(col).unwrap() == '*' {
        return -1;
    }
    println!("{} {}", row, col);

    let row_vecs: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let col_vecs: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1]; // usize vs i32 (자동 타입 추적이 usize로 인식)

    let row_size = minefield.len();
    let col_size = minefield[0].len();

    let mut mine_no = 0;
    for i in 0..row_vecs.len() {
        let row_vec = row_vecs[i];
        let col_vec = col_vecs[i];

        let target_row = (row as i32 + row_vec) as usize;
        let target_col = (col as i32 + col_vec) as usize;

        if target_row < 0 || target_col < 0 || target_row >= row_size || target_col >= col_size {
            continue;
        }

        // println!("{:?}", minefield);
        // println!("{:?}", minefield[target_row].chars());
        // println!("{:?} {} {}", minefield[target_row].chars().nth(target_col).unwrap(), target_row, target_col);


        if minefield[target_row].chars().nth(target_col).unwrap() == '*' {
            mine_no += 1;
        }
    }

    mine_no
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
