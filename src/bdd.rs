use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::rc::Rc;

type VarTableEntry = String;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct ReverseKey {
    var: u32,
    lo: u32,
    hi: u32,
}
impl ReverseKey {
    fn new(var: u32, lo: &BDD, hi: &BDD) -> Self {
        ReverseKey {
            var: var,
            lo: lo.node_number(),
            hi: hi.node_number(),
        }
    }
}

#[derive(Debug)]
pub struct BDDManager {
    var_table: Vec<VarTableEntry>,
    node_list: LinkedList<Rc<BDDNode>>,
    true_bdd: BDD,
    false_bdd: BDD,
    reverse_map: HashMap<ReverseKey, Rc<BDDNode>>,
}

#[derive(Debug)]
struct BDDNode {
    var: u32,
    node_number: u32,
    node_type: BDDNodeType,
}

#[derive(Debug)]
enum BDDNodeType {
    TrueNode,
    FalseNode,
    Node { lo: BDD, hi: BDD },
}

#[derive(Debug, Clone)]
pub struct BDD {
    head: Rc<BDDNode>,
}
type Operation = dyn Fn(bool, bool) -> bool;
impl BDDManager {
    pub fn new() -> Self {
        let f = Rc::new(BDDNode::false_node());
        let t = Rc::new(BDDNode::true_node());
        let mut node_list = LinkedList::new();
        node_list.push_back(Rc::clone(&f));
        node_list.push_back(Rc::clone(&t));

        BDDManager {
            var_table: vec![],
            node_list: node_list,
            true_bdd: BDD::new(t.clone()),
            false_bdd: BDD::new(f.clone()),
            reverse_map: HashMap::new(),
        }
    }
    fn mk(&mut self, var: u32, lo: &BDD, hi: &BDD) -> BDD {
        if var > self.var_table.len() as u32 {
            panic!("var shouldn't be larger than var table size");
        };

        if lo.node_number() == hi.node_number() {
            lo.clone()
        } else {
            let key = ReverseKey::new(var, lo, hi);
            match self.reverse_map.get(&key) {
                Some(bdd_node) => BDD::new(bdd_node.clone()),
                None => {
                    let new_node_number = self.node_list.len() as u32;
                    let new_node =
                        Rc::new(BDDNode::new(var, new_node_number, lo.clone(), hi.clone()));
                    self.node_list.push_back(new_node.clone());
                    self.reverse_map.insert(key, Rc::clone(&new_node));
                    BDD::new(new_node)
                }
            }
        }
    }
    pub fn var(&mut self, entry: VarTableEntry) -> BDD {
        self.var_table.push(entry);
        self.mk(
            self.var_table.len() as u32,
            &self.false_bdd.clone(),
            &self.true_bdd.clone(),
        )
    }
    fn apply(&mut self, fkt: &Operation, x: &BDD, y: &BDD) -> BDD {
        if x.is_constant() && y.is_constant() {
            if fkt(x.is_true(), y.is_true()) {
                self.true_bdd.clone()
            } else {
                self.false_bdd.clone()
            }
        } else if x.var() == y.var() {
            let lo = self.apply(fkt, x.low().unwrap(), y.low().unwrap());
            let hi = self.apply(fkt, x.high().unwrap(), y.high().unwrap());
            self.mk(x.var(), &lo, &hi)
        } else if x.var() < y.var() {
            let lo = self.apply(fkt, x.low().unwrap(), y);
            let hi = self.apply(fkt, x.high().unwrap(), y);
            self.mk(x.var(), &lo, &hi)
        } else {
            /* x.var() > y.var() */
            let lo = self.apply(fkt, x, y.low().unwrap());
            let hi = self.apply(fkt, x, y.high().unwrap());
            self.mk(y.var(), &lo, &hi)
        }
    }
    // TODO: implement operator overloads
    pub fn and_op(&mut self, x: &BDD, y: &BDD) -> BDD {
        self.apply(&|a, b| a & b, x, y)
    }
    pub fn or_op(&mut self, x: &BDD, y: &BDD) -> BDD {
        self.apply(&|a, b| a | b, x, y)
    }
    pub fn xor_op(&mut self, x: &BDD, y: &BDD) -> BDD {
        self.apply(&|a, b| a ^ b, x, y)
    }
    pub fn eq_op(&mut self, x: &BDD, y: &BDD) -> BDD {
        self.apply(&|a, b| a == b, x, y)
    }
    pub fn not_op(&mut self, x: &BDD) -> BDD {
        let t = self.true_bdd.clone();
        self.xor_op(x, &t)
    }
    fn node_name(node_number: u32) -> String {
        "n".to_string() + &node_number.to_string()
    }
    fn var_name(var_number: u32) -> String {
        "v".to_string() + &var_number.to_string()
    }
    fn dump_tikz_node_impl(&self, x: &BDD, table: &mut Vec<u32>, ret: &mut String) {
        if x.is_constant() {
            return;
        }
        let node_number = x.node_number();
        match ret.find(&(Self::node_name(node_number) + ")")) {
            Some(_) => return,
            _ => {}
        };
        let var = x.var();
        let left_node = table[var as usize];
        ret.push_str("    \\node[xshift=0cm, BDDnode, ");
        if left_node == u32::max_value() {
            ret.push_str(&("right of=".to_string() + &Self::var_name(var)));
        } else {
            ret.push_str(&("right of=".to_string() + &Self::node_name(left_node)));
        }
        ret.push_str("] (");
        ret.push_str(&Self::node_name(node_number));
        ret.push_str(") {\\small $");
        ret.push_str(&node_number.to_string());
        ret.push_str("$};\n");
        table[var as usize] = node_number;

        if x.low().is_some() {
            self.dump_tikz_node_impl(x.low().unwrap(), table, ret);
        }
        if x.high().is_some() {
            self.dump_tikz_node_impl(x.high().unwrap(), table, ret);
        }
    }
    fn dump_tikz_edge_impl(&self, x: &BDD, ret: &mut String, done: &mut HashSet<u32>) {
        if x.is_constant() {
            return;
        }
        if done.contains(&x.node_number()) {
            return;
        } else {
            done.insert(x.node_number());
        }
        let node_number = x.node_number();
        if x.low().is_some() {
            let lo = x.low().unwrap();
            let lo_node_number = lo.node_number();
            if !lo.is_false() {
                ret.push_str("    \\draw[->,dashed] (");
                ret.push_str(&Self::node_name(node_number));
                ret.push_str(") -> (");
                ret.push_str(&Self::node_name(lo_node_number));
                ret.push_str(");\n");
            }
            self.dump_tikz_edge_impl(lo, ret, done);
        }
        if x.high().is_some() {
            let hi = x.high().unwrap();
            let hi_node_number = hi.node_number();
            if !hi.is_false() {
                ret.push_str("    \\draw[->       ] (");
                ret.push_str(&Self::node_name(node_number));
                ret.push_str(") -> (");
                ret.push_str(&Self::node_name(hi_node_number));
                ret.push_str(");\n");
            }
            self.dump_tikz_edge_impl(hi, ret, done);
        }
    }
    pub fn dump_tikz(&self, x: &BDD) -> String {
        // TODO: implement!
        let mut ret = String::new();
        ret.reserve(1024 * 1024); // 適当に1MB malloc
        ret.push_str("\\begin{tikzpicture}[node distance=1cm]\n");
        ret.push_str(
            "    \\tikzstyle{BDDnode}=[circle,draw=black,inner sep=0pt,minimum size=5mm]\n",
        );
        ret.push_str("    % left nodes\n");
        for (node_number, node_name) in self.var_table.iter().enumerate() {
            ret.push_str("    \\node[");
            if node_number != 0 {
                ret.push_str("below of=");
                ret.push_str(&Self::var_name(node_number as u32));
            } else {
                ret.push_str("           ");
            }
            ret.push_str("] (");
            ret.push_str(&Self::var_name(node_number as u32 + 1));
            ret.push_str(") {$\\mathit{");
            ret.push_str(node_name);
            ret.push_str("}$};\n");
        }
        let mut table = vec![u32::max_value(); self.var_table.len() + 1];
        self.dump_tikz_node_impl(x, &mut table, &mut ret);
        ret.push_str("\n    % terminals\n");
        ret.push_str("    \\node[draw=black, style=rectangle, below of=");
        ret.push_str(&(Self::var_name(self.var_table.len() as u32)));
        ret.push_str(", xshift=1.5cm] (n0) {$0$};\n");
        ret.push_str("    \\node[draw=black, style=rectangle, right of=n0] (n1) {$1$};\n");
        ret.push_str("\n    % edges\n");
        self.dump_tikz_edge_impl(x, &mut ret, &mut HashSet::new());
        ret.push_str("\\end{tikzpicture}\n");

        ret.shrink_to_fit();
        ret
    }
    pub fn false_bdd(&self) -> BDD {
        self.false_bdd.clone()
    }
    pub fn true_bdd(&self) -> BDD {
        self.true_bdd.clone()
    }
}

impl BDD {
    fn new(bddnode: Rc<BDDNode>) -> Self {
        BDD { head: bddnode }
    }
    fn var(&self) -> u32 {
        self.head.var
    }
    fn node_number(&self) -> u32 {
        self.head.node_number
    }
    fn is_constant(&self) -> bool {
        match self.head.node_type {
            BDDNodeType::TrueNode => true,
            BDDNodeType::FalseNode => true,
            _ => false,
        }
    }
    fn is_true(&self) -> bool {
        match self.head.node_type {
            BDDNodeType::TrueNode => true,
            _ => false,
        }
    }
    fn is_false(&self) -> bool {
        match self.head.node_type {
            BDDNodeType::FalseNode => true,
            _ => false,
        }
    }
    fn low(&self) -> Option<&BDD> {
        match &self.head.node_type {
            BDDNodeType::Node { lo, hi: _hi } => Some(lo),
            _ => None,
        }
    }
    fn high(&self) -> Option<&BDD> {
        match &self.head.node_type {
            BDDNodeType::Node { lo: _lo, hi } => Some(hi),
            _ => None,
        }
    }
    pub fn count_answers(&self, var_num: u32) -> u32 {
        if self.is_true() {
            1
        } else if self.is_false() {
            0
        } else {
            let lo = self.low().unwrap();
            let lo_diff = if lo.is_constant() {
                var_num + 1 - self.var() - 1
            } else {
                lo.var() - self.var() - 1
            };
            let hi = self.high().unwrap();
            let hi_diff = if hi.is_constant() {
                var_num + 1 - self.var() - 1
            } else {
                hi.var() - self.var() - 1
            };
            (lo.count_answers(var_num) << lo_diff) + (hi.count_answers(var_num) << hi_diff)
        }
    }
    fn node_set_impl(&self, s: &mut HashSet<u32>) {
        if self.is_constant() {
            s.insert(self.node_number());
        } else {
            s.insert(self.node_number());
            let lo = self.low().unwrap();
            let hi = self.high().unwrap();
            lo.node_set_impl(s);
            hi.node_set_impl(s);
        }
    }
    fn node_set(&self) -> HashSet<u32> {
        // true_node, false_node も含める
        let mut s = HashSet::new();
        s.reserve(1024 * 1024);
        self.node_set_impl(&mut s);
        s.shrink_to_fit();

        s
    }
    pub fn count_nodes(&self) -> u32 {
        self.node_set().len() as u32
    }
}

impl BDDNode {
    fn new(var: u32, node_number: u32, lo: BDD, hi: BDD) -> Self {
        BDDNode {
            var: var,
            node_number: node_number,
            node_type: BDDNodeType::Node { lo: lo, hi: hi },
        }
    }
    fn false_node() -> Self {
        BDDNode {
            var: u32::max_value(),
            node_number: 0,
            node_type: BDDNodeType::FalseNode,
        }
    }
    fn true_node() -> Self {
        BDDNode {
            var: u32::max_value(),
            node_number: 1,
            node_type: BDDNodeType::TrueNode,
        }
    }
}
