mod tests {
    use crate::gusfields::{get_center_string, sort_edges};

    #[test]
    fn minimizing_center_string() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();

        adjacency_matrix.push([0, 1, 1, 1].to_vec());
        adjacency_matrix.push([1, 0, 2, 2].to_vec());
        adjacency_matrix.push([1, 2, 0, 2].to_vec());
        adjacency_matrix.push([1, 2, 2, 0].to_vec());
        
        assert_eq!(0, get_center_string(&adjacency_matrix, false));
    }

    #[test]
    fn maximizing_center_string() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        
        adjacency_matrix.push([0, -1, -2, -2].to_vec());
        adjacency_matrix.push([-1, 0, -1, -1].to_vec());
        adjacency_matrix.push([-2, -1, 0, -2].to_vec());
        adjacency_matrix.push([-2, -1, -2, 0].to_vec());
        
        assert_eq!(1, get_center_string(&adjacency_matrix, true));
    }
    
    #[test]
    fn minimized_order_kruskal_edges() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        
        adjacency_matrix.push([0, 2, 8, 1].to_vec());
        adjacency_matrix.push([2, 0, 0, 11].to_vec());
        adjacency_matrix.push([8, 0, 0, 9].to_vec());
        adjacency_matrix.push([1, 11, 9, 0].to_vec());
        
        let sorted_edges = sort_edges(&adjacency_matrix, false).unwrap();
        
        assert_eq!(sorted_edges[0], (1, 2));
        assert_eq!(sorted_edges[1], (0, 3));
        assert_eq!(sorted_edges[2], (0, 1));
        assert_eq!(sorted_edges[3], (0, 2));
        assert_eq!(sorted_edges[4], (2, 3));
        assert_eq!(sorted_edges[5], (1, 3));
    }

    #[test]
    fn maximized_order_kruskal_edges() {
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        
        adjacency_matrix.push([0, 2, 8, 1].to_vec());
        adjacency_matrix.push([2, 0, 0, 11].to_vec());
        adjacency_matrix.push([8, 0, 0, 9].to_vec());
        adjacency_matrix.push([1, 11, 9, 0].to_vec());
        
        let sorted_edges = sort_edges(&adjacency_matrix, true).unwrap();
        
        assert_eq!(sorted_edges[5], (1, 2));
        assert_eq!(sorted_edges[4], (0, 3));
        assert_eq!(sorted_edges[3], (0, 1));
        assert_eq!(sorted_edges[2], (0, 2));
        assert_eq!(sorted_edges[1], (2, 3));
        assert_eq!(sorted_edges[0], (1, 3));
    }
}