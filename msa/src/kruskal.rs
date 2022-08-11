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

pub fn get_kruskal_mst(adjacency_matrix: &Vec<Vec<Alignment>>, maximize: bool) -> Option<Vec<(usize, usize)>> {
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

