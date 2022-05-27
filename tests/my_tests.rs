


macro_rules! board_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                test_cube($value, 100);
            }
        )*
        }
}

board_tests!(
 t1: "98_-7+524",
 t2: "7-6574+2/",
 t3: "-+718325+",
 t4: "7+58-2675",
 t5: "34+*2651+",
 t6: "813+*-372",
 t7: "-+718325+",
 t8: "6+98161-3",
 t9: "-9+1236+5",
t10: "+-389-425",
t11: "/3+421+58",

);

fn test_cube(_letters: &str, _expected_count: usize) {
    todo!()
}

