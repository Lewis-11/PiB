
pub fn get_center_string(adjacency_matrix: &Vec<Vec<i32>>, maximize: bool) -> usize {
    let n = adjacency_matrix.len();
    let mut center_string = 0;
    let mut max_score = if maximize { i32::MIN } else { i32::MAX };

    for i in 0..n {
        let mut score = 0;
        for j in 0..n {
            score += &adjacency_matrix[i][j];
        }
        if maximize && (&score > &max_score) {
            max_score = score;
            center_string = i;
        } else if !maximize && (&score < &max_score) {
            max_score = score;
            center_string = i;
        }
    }
    return center_string;
}

fn sort_edges(adjacency_matrix: &Vec<Vec<i32>>, maximize: bool) -> Option<Vec<(usize, usize)>> {
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
            adjacency_matrix[k][l].cmp(&adjacency_matrix[i][j])
        } else {
            adjacency_matrix[i][j].cmp(&adjacency_matrix[k][l])
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

pub fn merge_clusters(cl1: &Vec<Vec<u8>>, cl2: &Vec<Vec<u8>>, idx1: i32, idx2: i32, pairwise1: Vec<u8>, pairwise2: Vec<u8>) -> Option<Vec<Vec<u8>>> {
    // merge cluster 1 and cluster2.
    // index1 is the index of the representative sequence of cluster1
    // index2 is the index of the representative sequence of cluster2
    // pairwise1 and pairwise2 are the pairwise alignments between the representative sequences of cluster1 and cluster2
    let cl1_representative = &cl1[idx1 as usize];
    let cl2_representative = &cl2[idx2 as usize];
    let n1 = cl1.len();
    let n2 = cl2.len();
    let pairwise_length = pairwise1.len();

    let mut i = 0; // index of the representative sequence in cluster1
    let mut j = 0; // index of the representative sequence in cluster2
    let mut k = 0; // index of pairwise

    let mut cl3: Vec<Vec<u8>> = Vec::new();
    for _ in 0..n1+n2 {
        cl3.push(Vec::new());
    }

    while i < cl1_representative.len() && j < cl2_representative.len() && k < pairwise_length {
        if cl1_representative[i] == pairwise1[k] {
            if cl2_representative[j] == pairwise2[k] {
                for l in 0..n1 {
                    cl3[l].push(cl1[l][i]);
                }
                for l in 0..n2 {
                    cl3[l + n1].push(cl2[l][j]);
                }
                i += 1;
                j += 1;
                k += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                for l in 0..n1 {
                    cl3[l].push(b'-');
                }
                for l in 0..n2 {
                    cl3[l + n1].push(cl2[l][j]);
                }
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                for l in 0..n1 {
                    cl3[l].push(cl1[l][i]);
                }
                for l in 0..n2 {
                    cl3[l + n1].push(b'-');
                }
                i += 1;
                k += 1;
            }
        } else if cl1_representative[i] == b'-' && pairwise1[k] != b'-' {
            if cl2_representative[j] == pairwise2[k] {
                for l in 0..n1 {
                    cl3[l].push(cl1[l][i]);
                }
                for l in 0..n2 {
                    cl3[l + n1].push(b'-');
                }
                i += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                for l in 0..n1 {
                    cl3[l].push(cl1[l][i]);
                }
                for l in 0..n2 {
                    cl3[l + n1].push(cl2[l][j]);
                }
                i += 1;
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                for l in 0..n1 {
                    cl3[l].push(cl1[l][i]);
                }
                for l in 0..n2 {
                    cl3[l + n1].push(b'-');
                }
                i += 1;
            }
        }
        else if cl1_representative[i] != b'-' && pairwise1[k] == b'-' {
            if cl2_representative[j] == pairwise2[k] {
                for l in 0..n1 {
                    cl3[l].push(b'-');
                }
                for l in 0..n2 {
                    cl3[l + n1].push(cl2[l][j]);
                }
                j += 1;
                k += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                for l in 0..n1 {
                    cl3[l].push(b'-');
                }
                for l in 0..n2 {
                    cl3[l + n1].push(cl2[l][j]);
                }
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                // should not happen
                for l in 0..n1 {
                    cl3[l].push(b'-');
                }
                for l in 0..n2 {
                    cl3[l + n1].push(b'-');
                }
                k += 1;
            }
        }
    }

    while i < cl1_representative.len() {
        for l in 0..n1 {
            cl3[l].push(cl1[l][i]);
        }
        for l in 0..n2 {
            cl3[l + n1].push(b'-');
        }
        i += 1;
    }

    while j < cl2_representative.len() {
        for l in 0..n1 {
            cl3[l].push(b'-');
        }
        for l in 0..n2 {
            cl3[l + n1].push(cl2[l][j]);
        }
        j += 1;
    }

    return Some(cl3);
}

pub fn gusfield_mst(score_matrix: &Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let center_string = get_center_string(score_matrix, true);
    let n = score_matrix.len();
    let mut merge_order : Vec<(i32, i32)>= Vec::new();
    for i in 0..n {
        if i == center_string { continue; };
        merge_order.push((center_string as i32, i as i32));
    }
    return Some(merge_order);
}

pub fn kruskal_mst(adjacency_matrix: &Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let edges = sort_edges(adjacency_matrix, false).expect("[!] Kruskal error: Could not sort edges");
    let n = adjacency_matrix.len();

    let mut parents = (0..n).collect::<Vec<usize>>();
    let mut ranks = vec![0; n];

    let mut tree: Vec<(i32, i32)> = Vec::new();

    for (i, j) in edges {
        let x = find_set(i, &parents);
        let y = find_set(j, &parents);
        if x != y {
            union_sets(x, y, &mut parents, &mut ranks);
            tree.push((i as i32, j as i32));
        }
        if tree.len() == n-1 {
            break;
        }
    }
    return Some(tree);
}

#[cfg(test)]
#[path ="./tests/gusfields.rs"]
mod tests;