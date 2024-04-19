/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, start: usize,)-> Vec<usize>{
        //TODO
        let mut visit_order = vec![];
        visit_order.push(start);
        let mut charge = 0;
        if self.adj.len() == 0{
            visit_order
        }else{
            for i in 0..self.adj[start].len(){
                visit_order.push(self.adj[start][i]);
                charge += 1;
                if charge == self.adj.len(){
                    return visit_order;
                }
            }
            for i in 0..self.adj[start].len(){
                for j in 0..self.adj[self.adj[start][i]].len(){
                    if visit_order.iter().any(|&num| num == self.adj[self.adj[start][i]][j]){
                        continue;
                    }else{
                        visit_order.push(self.adj[self.adj[start][i]][j]);
                        charge += 1;
                        if charge == self.adj.len(){
                        return visit_order;
                        }
                    }
                }
            }
            visit_order
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        
        self.dfs_util(start)
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

