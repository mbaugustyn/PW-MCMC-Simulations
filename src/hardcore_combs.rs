fn clear_mat<const GRAPH_SIZE: usize>(
    x: usize,
    y: usize,
    mat: &mut [[bool; GRAPH_SIZE]; GRAPH_SIZE],
) {
    for j in y..GRAPH_SIZE {
        mat[x][j] = false;
    }
    for i in (x + 1)..GRAPH_SIZE {
        for j in 0..GRAPH_SIZE {
            mat[i][j] = false;
        }
    }
}
fn can_put<const GRAPH_SIZE: usize>(
    x: usize,
    y: usize,
    mat: &[[bool; GRAPH_SIZE]; GRAPH_SIZE],
) -> bool {
    return !(mat[x][y]
        || (y > 0 && mat[x][y - 1])
        || (y + 1 < GRAPH_SIZE && mat[x][y + 1])
        || (x > 0 && mat[x - 1][y])
        || (x + 1 < GRAPH_SIZE && mat[x + 1][y]));
}

pub fn put<const GRAPH_SIZE: usize>(
    x: usize,
    y: usize,
    mat: &mut [[bool; GRAPH_SIZE]; GRAPH_SIZE],
    solutions: &mut Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]>,
) {
    if can_put::<GRAPH_SIZE>(x, y, mat) {
        mat[x][y] = true;
        solutions.push(mat.clone());
        if y + 1 < GRAPH_SIZE {
            put::<GRAPH_SIZE>(x, y + 1, mat, solutions);
        } else if x + 1 < GRAPH_SIZE {
            put::<GRAPH_SIZE>(x + 1, 0, mat, solutions);
        } else {
            return;
        }
    }

    mat[x][y] = false;
    clear_mat::<GRAPH_SIZE>(x, y, mat);
    if y + 1 < GRAPH_SIZE {
        put::<GRAPH_SIZE>(x, y + 1, mat, solutions);
    } else if x + 1 < GRAPH_SIZE {
        put::<GRAPH_SIZE>(x + 1, 0, mat, solutions);
    } else {
        return;
    }
}

/* Returns a vector of all of the possible feasible combinationsw */
pub fn hardcore_combs<const GRAPH_SIZE: usize>() -> Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]> {
    let solutions: &mut Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]> = &mut vec![];
    let mat = &mut [[false; GRAPH_SIZE]; GRAPH_SIZE];
    solutions.push(mat.clone());
    put::<GRAPH_SIZE>(0, 0, mat, solutions);
    return solutions.clone();
}
