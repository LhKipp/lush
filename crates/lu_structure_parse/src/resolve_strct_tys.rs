use lu_interpreter_structs::*;

#[allow(dead_code)]
fn resolve_strct_tys(modules: &mut Vec<ScopeFrame<Variable>>) {
    let mut strcts_to_resolve = vec![];
    let mut funcs_to_resolve = vec![];

    for frame in modules {
        for (_, var) in frame.elems.iter_mut() {
            if let Value::StrctDecl(strct) = &var.val {
                strcts_to_resolve.push(strct.clone());
            } else if let Value::Command(func) = &var.val {
                funcs_to_resolve.push(func.clone());
            }
        }
    }

    todo!();
}
