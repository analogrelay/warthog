#![deny(warnings)]

extern crate warthog;

macro_rules! spec_test {
    ($name: ident) => {
        #[test]
        pub fn $name() {
            $crate::run_spec_test(stringify!($name));
        }
    };
}

mod spec_tests {
    spec_test!(i32);
    spec_test!(i64);

    spec_test!(comments);
}

fn run_spec_test(name: &'static str) {
    let mut parent_dir = env::current_dir().unwrap();
    parent_dir.push("vendor");
    parent_dir.push("webassembly");
    parent_dir.push("test");
    parent_dir.push("core");

    if !parent_dir.exists() {
        panic!(
            "Could not find WebAssembly spec tests directory: '{}'",
            parent_dir.to_str().unwrap()
        );
    }

    let mut test_path = parent_dir.clone();
    test_path.push(format!("{}.wast", name));

    if !test_path.exists() {
        panic!(
            "Could not find WebAssembly spec test '{}.wast' in '{}'",
            name,
            parent_dir.to_str().unwrap()
        );
    }

    let commands = {
        let mut file = fs::File::open(test_path).expect("failed to open file");
        text::parse(&mut file).unwrap()
    };

    run(commands);
}
