use super::LuNativeStdMod;

pub(crate) struct IterFuncsMod {}

impl LuNativeStdMod for IterFuncsMod {
    fn id(&self) -> String {
        "std:iter".into()
    }
    fn src(&self) -> &str {
        r#"
        use std:array

        fn map (in: [T] ret: [U] map_fn: fn(ret:U arg: T))
            let result = []
            for v in $in
                $result = push $result (map_fn $v)
            end
            ret $result
        end

        fn filter (in: [T] ret: [T] filter_fn: fn(ret: bool arg: T))
            let result = []
            for v in $in
                if filter_fn $v
                    $result = push $result $v
                end
            end
            ret $result
        end
        "#
    }
}
