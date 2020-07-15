use bdd::bdd::BDDManager;
use std::collections::HashMap;

pub struct Graph {
    node_entry: HashMap<String, usize>,
    edge: Vec<Vec<usize>>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            node_entry: HashMap::new(),
            edge: vec![],
        }
    }
    pub fn add_node(&mut self, node_name: &str) {
        let new_node_number = self.node_entry.len() as usize;
        self.node_entry
            .insert(node_name.to_string(), new_node_number);
        self.edge.push(vec![]);
    }
    pub fn add_edge(&mut self, node_a: &str, node_b: &str) -> Result<(), ()> {
        match (self.node_entry.get(node_a), self.node_entry.get(node_b)) {
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
    pub fn nodes(&self) -> &HashMap<String, usize> {
        &self.node_entry
    }
    pub fn edges(&self) -> &Vec<Vec<usize>> {
        &self.edge
    }
}

pub fn united_states() -> Graph {
    let mut g = Graph::new();
    let nodes = [
        "CA", "WA", "OR", "NV", "ID", "UT", "AZ", "MT", "WY", "CO", "NM", "ND", "SD", "NE", "KS",
        "OK", "TX", "MN", "IA", "MO", "AR", "LA", "WI", "IL", "MS", "MI", "IN", "KY", "TN", "AL",
        "OH", "WV", "VA", "GA", "FL", "PA", "MD", "DC", "NC", "SC", "VT", "NY", "NJ", "DE", "NH",
        "MA", "CT", "ME", "RI",
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
    for (n, id) in us.nodes().iter() {
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
