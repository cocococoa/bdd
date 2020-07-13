use bdd::bdd::BDDManager;

#[test]
fn test_c6() {
    let mut mgr = BDDManager::new();
    let vertex = 6;
    let mut vertices = vec![];
    for i in 0..vertex {
        let v = "v".to_string() + &(i + 1).to_string();
        vertices.push(mgr.var(v));
    }
    let mut f = mgr.and_op(&vertices[0], &vertices[vertex - 1]);
    for i in 0..(vertex - 1) {
        let x = mgr.and_op(&vertices[i], &vertices[i + 1]);
        f = mgr.or_op(&f, &x);
    }
    let independence = mgr.not_op(&f);
    assert_eq!(independence.count_answer(), 18);
    println!("independent: \n{}", mgr.dump_tikz(&independence));

    for i in 0..vertex {
        let mut x = vertices[i].clone();
        let pre_ind = (i + vertex - 1) % vertex;
        let aft_ind = (i + 1) % vertex;
        x = mgr.or_op(&x, &vertices[pre_ind]);
        x = mgr.or_op(&x, &vertices[aft_ind]);
        if i == 0 {
            f = x;
        } else {
            f = mgr.and_op(&f, &x);
        }
    }
    let kernel_impl = f;
    let kernel = mgr.and_op(&independence, &kernel_impl);
    assert_eq!(kernel.count_answer(), 5);
    println!("kernel: \n{}", mgr.dump_tikz(&kernel));
}
