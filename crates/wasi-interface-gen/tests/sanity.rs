use wasi_interface_gen::wasi_interface;

#[wasi_interface]
mod foo {
    struct Input {
        s: String,
        i: i64,
    }

    struct Output {
        a: i64,
        b: f64,
        c: String,
    }

    fn mapper(input: Input) -> Vec<Output> {
        vec![Output {
            a: input.i * input.i,
            b: (input.i as f64) * 123.234,
            c: format!("hello {}", input.s),
        }]
    }
}

#[test]
fn sanity() {}
