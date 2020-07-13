use bdd::bdd::BDDManager;

fn main() {
    let mut mgr = BDDManager::new();
    let a = mgr.var("a".to_string());
    let b = mgr.var("b".to_string());
    let c = mgr.var("c".to_string());
    let d = mgr.var("d".to_string());
    let f1 = mgr.eq_op(&a, &b);
    let f2 = mgr.eq_op(&c, &d);
    let f = mgr.and_op(&f1, &f2);
    // println!("{:?}", f);
    println!("{}", mgr.dump_tikz(&f));
}
