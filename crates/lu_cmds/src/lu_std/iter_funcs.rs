use super::LuNativeStdMod;

pub(crate) struct IterFuncsMod {}

impl LuNativeStdMod for IterFuncsMod {
    fn id(&self) -> Vec<String> {
        ["std".to_string(), "iter".to_string()].to_vec()
    }
    fn src(&self) -> &str {
        r#"
        use std:array

        fn map (in: [T] ret: [U] map_fn: fn(ret:U arg: T))
            let result = []
            for v in $in
                let v_u = map_fn $v
                let new_arr = push $result $v_u
                $result = $new_arr
            end
            ret $result
        end
        "#
    }
}
