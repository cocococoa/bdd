use bdd::bdd::BDDManager;
use std::collections::HashMap;

pub struct Graph {
    node_entry: Vec<String>,
    edge: Vec<Vec<usize>>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            node_entry: vec![],
            edge: vec![],
        }
    }
    pub fn add_node(&mut self, node_name: &str) {
        self.node_entry.push(node_name.to_string());
        self.edge.push(vec![]);
    }
    fn get_id(&self, node: &str) -> Option<usize> {
        let mut ret = usize::max_value();
        for i in 0..self.node_entry.len() {
            if self.node_entry[i] == node {
                ret = i;
                break;
            }
        }

        if ret == usize::max_value() {
            None
        } else {
            Some(ret)
        }
    }
    pub fn add_edge(&mut self, node_a: &str, node_b: &str) -> Result<(), ()> {
        match (self.get_id(node_a), self.get_id(node_b)) {
            (Some(id_a), Some(id_b)) => {
                let id_a = id_a.clone();
                let id_b = id_b.clone();
                self.edge[id_a].push(id_b);
                self.edge[id_b].push(id_a);
                Ok(())
            }
            _ => Err(()),
        }
    }
    pub fn nodes(&self) -> &Vec<String> {
        &self.node_entry
    }
    pub fn edges(&self) -> &Vec<Vec<usize>> {
        &self.edge
    }
}

pub fn united_states() -> Graph {
    let mut g = Graph::new();
    // 410, 784
    let nodes = [
        "ME", "NH", "VT", "MA", "RI", "CT", "NY", "NJ", "DE", "PA", "MD", "DC", "OH", "WV", "VA",
        "NC", "SC", "GA", "FL", "AL", "MI", "IN", "KY", "TN", "MS", "IL", "WI", "MN", "IA", "MO",
        "AR", "LA", "TX", "OK", "NE", "KS", "SD", "ND", "WY", "MT", "CO", "NM", "AZ", "ID", "UT",
        "NV", "CA", "WA", "OR",
    ];
    assert_eq!(nodes.len(), 49);
    let edges = [
        ("CA", "OR"),
        ("CA", "NV"),
        ("CA", "AZ"),
        ("WA", "OR"),
        ("WA", "ID"),
        ("OR", "NV"),
        ("OR", "ID"),
        ("NV", "ID"),
        ("NV", "UT"),
        ("NV", "AZ"),
        ("ID", "UT"),
        ("ID", "MT"),
        ("ID", "WY"),
        ("UT", "AZ"),
        ("UT", "WY"),
        ("UT", "CO"),
        ("AZ", "NM"),
        ("MT", "WY"),
        ("MT", "ND"),
        ("MT", "SD"),
        ("WY", "CO"),
        ("WY", "SD"),
        ("WY", "NE"),
        ("CO", "NM"),
        ("CO", "NE"),
        ("CO", "KS"),
        ("CO", "OK"),
        ("NM", "OK"),
        ("NM", "TX"),
        ("ND", "SD"),
        ("ND", "MN"),
        ("SD", "NE"),
        ("SD", "MN"),
        ("SD", "IA"),
        ("NE", "KS"),
        ("NE", "IA"),
        ("NE", "MO"),
        ("KS", "OK"),
        ("KS", "MO"),
        ("OK", "TX"),
        ("OK", "MO"),
        ("OK", "AR"),
        ("TX", "AR"),
        ("TX", "LA"),
        ("MN", "IA"),
        ("MN", "WI"),
        ("IA", "MO"),
        ("IA", "WI"),
        ("IA", "IL"),
        ("MO", "AR"),
        ("MO", "IL"),
        ("MO", "KY"),
        ("MO", "TN"),
        ("AR", "LA"),
        ("AR", "MS"),
        ("AR", "TN"),
        ("LA", "MS"),
        ("WI", "IL"),
        ("WI", "MI"),
        ("IL", "IN"),
        ("IL", "KY"),
        ("MS", "TN"),
        ("MS", "AL"),
        ("MI", "IN"),
        ("MI", "OH"),
        ("IN", "KY"),
        ("IN", "OH"),
        ("KY", "TN"),
        ("KY", "OH"),
        ("KY", "WV"),
        ("KY", "VA"),
        ("TN", "AL"),
        ("TN", "VA"),
        ("TN", "GA"),
        ("TN", "NC"),
        ("AL", "GA"),
        ("AL", "FL"),
        ("OH", "WV"),
        ("OH", "PA"),
        ("WV", "VA"),
        ("WV", "PA"),
        ("WV", "MD"),
        ("VA", "MD"),
        ("VA", "DC"),
        ("VA", "NC"),
        ("GA", "FL"),
        ("GA", "NC"),
        ("GA", "SC"),
        ("PA", "MD"),
        ("PA", "NY"),
        ("PA", "NJ"),
        ("PA", "DE"),
        ("MD", "DC"),
        ("MD", "DE"),
        ("NC", "SC"),
        ("VT", "NY"),
        ("VT", "NH"),
        ("VT", "MA"),
        ("NY", "NJ"),
        ("NY", "MA"),
        ("NY", "CT"),
        ("NJ", "DE"),
        ("NH", "MA"),
        ("NH", "ME"),
        ("MA", "CT"),
        ("MA", "RI"),
        ("CT", "RI"),
    ];
    assert_eq!(edges.len(), 107);

    for n in nodes.iter() {
        g.add_node(n);
    }
    for (a, b) in edges.iter() {
        let res = g.add_edge(a, b);
        if res.is_err() {
            println!("[ERROR] a: {}, b: {}", a, b);
        }
    }

    g
}

#[test]
fn test_us() {
    let us = united_states();
    let mut mgr = BDDManager::new();

    // 変数の発行
    let mut vertices = HashMap::new();
    for (id, n) in us.nodes().iter().enumerate() {
        let v = mgr.var(n.to_string());
        vertices.insert(id, v);
    }

    // 独立集合の条件
    let mut f = mgr.false_bdd();
    for (u, next_vec) in us.edges().iter().enumerate() {
        for v in next_vec.iter() {
            if u < *v {
                let x = mgr.and_op(vertices.get(&u).unwrap(), vertices.get(v).unwrap());
                f = mgr.or_op(&f, &x);
            }
        }
    }
    let mut f = mgr.not_op(&f);
    assert_eq!(f.count_answers(vertices.len() as u32), 211_954_906);
    // TODO: 現状は最小のBDDを生み出すノードの順番が分からないため通らない
    // assert_eq!(f.count_nodes(), 428);

    // カーネルの条件
    for (u, next_vec) in us.edges().iter().enumerate() {
        let mut x = vertices.get(&u).unwrap().clone();
        for v in next_vec.iter() {
            x = mgr.or_op(&x, vertices.get(v).unwrap());
        }
        f = mgr.and_op(&f, &x);
    }
    assert_eq!(f.count_answers(vertices.len() as u32), 266_137);
    // TODO: 現状は最小のBDDを生み出すノードの順番が分からないため通らない
    // assert_eq!(kernel.count_nodes(), 780);
}
