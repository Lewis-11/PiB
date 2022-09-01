use crate::fasta::Alignment;

fn sort_edges(adjacency_matrix: &Vec<Vec<Alignment>>, maximize: bool) -> Option<Vec<(usize, usize)>> {
    let n = adjacency_matrix.len();
    if n == 0 {
        return None;
    }
    let mut edges = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            edges.push((i, j));
        }
    }
    edges.sort_by(|&(i, j), &(k, l)| {
        if maximize {
            adjacency_matrix[k][l].score.cmp(&adjacency_matrix[i][j].score)
        } else {
            adjacency_matrix[i][j].score.cmp(&adjacency_matrix[k][l].score)
        }
    });
    return Some(edges);
}

fn find_set(i: usize, parents: &Vec<usize>) -> usize {
    if parents[i] == i {
        return i;
    }
    return find_set(parents[i], parents);
}

fn union_sets(x:usize, y:usize, parents: &mut Vec<usize>, ranks: &mut Vec<usize>) {
    if ranks[x] < ranks[y] {
        parents[x] = y;
    } else if ranks[x] > ranks[y] {
        parents[y] = x;
    } else {
        parents[y] = x;
        ranks[x] += 1;
    }
}

fn get_kruskal_mst(adjacency_matrix: &Vec<Vec<Alignment>>, maximize: bool) -> Option<Vec<(usize, usize)>> {
    let edges = sort_edges(adjacency_matrix, maximize).expect("[!] Kruskal error: Could not sort edges");
    let n = adjacency_matrix.len();

    let mut parents = (0..n).collect::<Vec<usize>>();
    let mut ranks = vec![0; n];

    let mut tree = Vec::new();

    for (i, j) in edges {
        let x = find_set(i, &parents);
        let y = find_set(j, &parents);
        if x != y {
            union_sets(x, y, &mut parents, &mut ranks);
            tree.push((i, j));
        }
        if tree.len() == n-1 {
            break;
        }
    }
    return Some(tree);
}

fn insert_gap_at(matrix: &mut Vec<Vec<u8>>, index: usize) {
    for row in matrix.iter_mut() {
        row.insert(index, b'-');
    }
}

fn insert_sequence(matrix: &mut Vec<Vec<u8>>, sequences: &Alignment, compare_index: usize) {
    let insert_seq = sequences.sequences[0].sequence.as_bytes().to_vec();
    let compare_seq = sequences.sequences[compare_index].sequence.as_bytes().to_vec();
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut new_row = Vec::new();
    while i < matrix[0].len() && j < compare_seq.len() {

        // Case 1:
        if matrix[compare_index][i] == compare_seq[j] {
            new_row.push(insert_seq[j]);
            i += 1;
            j += 1;
        }

        // Case 2:
        else if matrix[compare_index][i] == b'-' && compare_seq[j] != b'-' {
            new_row.push(b'-');
            i += 1;
        }

        // Case 3:
        else if matrix[compare_index][i] != b'-' && compare_seq[j] == b'-' {
            insert_gap_at(matrix, i);
            new_row.push(insert_seq[j]);
            i += 1;
            j += 1;
        }
    }
    while j < compare_seq.len() {
        insert_gap_at(matrix, i);
        new_row.push(insert_seq[j]);
        i += 1;
        j += 1;
    }
    while i < matrix[compare_index].len() {
        new_row.push(b'-');
        i += 1;
    }
    matrix.push(new_row);
}

pub(crate) fn kruskal_sorted_edges_alignment(adjacency_matrix: &Vec<Vec<Alignment>>, maximize: bool) -> Vec<Vec<u8>> {
    let tree = get_kruskal_mst(adjacency_matrix, maximize).expect("[!] Kruskal error: Could not get MST");
    let n = adjacency_matrix.len();
    let mut matrix = Vec::new();
    let mut inserted = vec![n; n];
    matrix.push(adjacency_matrix[tree[0].0][tree[0].1].sequences[0].sequence.as_bytes().to_vec());
    inserted[tree[0].0] = 0;
    while matrix.len() < n {
        for (i, j) in &tree {
            if inserted[*i] != n && inserted[*j] == n {
                insert_sequence(&mut matrix, &adjacency_matrix[*j][*i], inserted[*i]);
                inserted[*j] = matrix.len()-1;
            } else if inserted[*i] == n && inserted[*j] != n {
                insert_sequence(&mut matrix, &adjacency_matrix[*i][*j], inserted[*j]);
                inserted[*i] = matrix.len()-1;
            }
        }
    }
    return matrix;
}


