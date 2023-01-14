mod tests {
    use crate::gusfields::get_center_string;

    #[test]
    fn minimizing_center_string() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        
        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(0);
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(1);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(0);
        adjacency_matrix.last_mut().unwrap().push(2);
        adjacency_matrix.last_mut().unwrap().push(2);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(2);
        adjacency_matrix.last_mut().unwrap().push(0);
        adjacency_matrix.last_mut().unwrap().push(2);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(2);
        adjacency_matrix.last_mut().unwrap().push(2);
        adjacency_matrix.last_mut().unwrap().push(0);
        
        assert_eq!(0, get_center_string(&adjacency_matrix, false));
    }

    #[test]
    fn maximizing_center_string() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        
        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(0);
        adjacency_matrix.last_mut().unwrap().push(-1);
        adjacency_matrix.last_mut().unwrap().push(-2);
        adjacency_matrix.last_mut().unwrap().push(-2);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(-1);
        adjacency_matrix.last_mut().unwrap().push(0);
        adjacency_matrix.last_mut().unwrap().push(-1);
        adjacency_matrix.last_mut().unwrap().push(-1);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(-2);
        adjacency_matrix.last_mut().unwrap().push(1);
        adjacency_matrix.last_mut().unwrap().push(-0);
        adjacency_matrix.last_mut().unwrap().push(-2);

        adjacency_matrix.push(Vec::new());
        adjacency_matrix.last_mut().unwrap().push(-2);
        adjacency_matrix.last_mut().unwrap().push(-1);
        adjacency_matrix.last_mut().unwrap().push(-2);
        adjacency_matrix.last_mut().unwrap().push(0);
        
        assert_eq!(1, get_center_string(&adjacency_matrix, true));
    }
}