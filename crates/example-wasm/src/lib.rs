use wasi_interface_gen::wasi_interface;

#[wasi_interface]
mod component {
    struct SimpleValue {
        i: i64,
    }

    fn double(input: SimpleValue) -> Vec<SimpleValue> {
        vec![SimpleValue { i: input.i * 2 }]
    }

    struct SplitInput {
        s: String,
        delimiter: String,
    }

    struct SplitOutput {
        c: String,
    }

    fn split(input: SplitInput) -> Vec<SplitOutput> {
        input
            .s
            .split(&input.delimiter)
            .map(|s| SplitOutput { c: s.to_string() })
            .collect()
    }

    struct User {
        id: i64,
        username: String,
        email: String,
        phone: String,
    }

    static BAD_DOMAINS: &[&str] = &["example.com", "example.net", "example.org"];

    fn filter_out_bad_users(input: User) -> Vec<User> {
        if BAD_DOMAINS
            .iter()
            .any(|domain| input.email.ends_with(domain))
        {
            vec![]
        } else {
            vec![input]
        }
    }
}
