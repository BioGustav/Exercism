pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return Vec::new();
    }

    minefield
        .iter()
        .enumerate()
        .map(|(r, row)| {
            String::from_utf8(
                row.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(c, col)| {
                        if *col == b'*' {
                            b'*'
                        } else {
                            count_mines(r, c, minefield)
                        }
                    })
                    .collect::<Vec<u8>>(),
            )
            .unwrap_or_default()
        })
        .collect()

    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
}

fn count_mines(r: usize, c: usize, minefield: &[&str]) -> u8 {
    let r_from = r.checked_sub(1).unwrap_or(0);
    let c_from = c.checked_sub(1).unwrap_or(0);

    let r_to = (r + 2).min(minefield.len());
    let c_to = (c + 2).min(minefield[0].len());

    let mut counter = 0;
    for r in r_from..r_to {
        for c in c_from..c_to {
            if minefield[r].as_bytes()[c] == b'*' {
                counter += 1;
            }
        }
    }

    if counter == 0 {
        b' '
    } else {
        {
            counter + b'0'
        }
    }
}
