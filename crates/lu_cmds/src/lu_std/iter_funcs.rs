use super::LuNativeStdMod;

pub(crate) struct IterFuncsMod {}

impl LuNativeStdMod for IterFuncsMod {
    fn id(&self) -> Vec<String> {
        ["std".to_string(), "iter".to_string()].to_vec()
    }
    fn src(&self) -> &str {
        r#"
        fn map (in: [T] ret: [U] map_fn fn(in:T ret:U))
            let result = []
            for v in $in
                let v_u = map_fn $v
                print v_u
            end
        end
        "#
    }
}
