use std::convert::TryInto;
use wasi_interface_gen::wasi_interface;

#[wasi_interface]
mod component {

    struct SimpleString {
        s: String,
    }

    struct PolarityScores {
        compound: f64,
        positive: f64,
        negative: f64,
        neutral: f64,
    }

    fn sentiment(input: SimpleString) -> PolarityScores {
        lazy_static::lazy_static! {
            static ref ANALYZER: vader_sentiment::SentimentIntensityAnalyzer<'static> =
                vader_sentiment::SentimentIntensityAnalyzer::new();
        }

        let scores = ANALYZER.polarity_scores(input.s.as_str());
        PolarityScores {
            compound: scores["compound"],
            positive: scores["pos"],
            negative: scores["neg"],
            neutral: scores["neu"],
        }
    }

    struct SimpleValue {
        i: i64,
    }

    fn square(input: SimpleValue) -> Vec<SimpleValue> {
        vec![SimpleValue {
            i: input.i * input.i,
        }]
    }

    struct TokenizeInput {
        s: String,
        delimiter: String,
    }

    struct TokenizeOutput {
        c: String,
    }

    fn tokenize(input: TokenizeInput) -> Vec<TokenizeOutput> {
        input
            .s
            .split(&input.delimiter)
            .map(|s| TokenizeOutput { c: s.to_string() })
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

    struct HilbertInput {
        vec: Vec<u8>,
        min_value: f64,
        max_value: f64,
        scale: f64,
    }
    struct HilbertOutput {
        idx: String,
    }

    fn hilbert_encode(input: HilbertInput) -> Vec<HilbertOutput> {
        let range = hilbert::FloatDataRange::new(input.min_value, input.max_value, input.scale);

        let raw_point = super::vector_unpack(&input.vec)
            .into_iter()
            .map(|x| range.normalize(x))
            .collect::<Vec<u32>>();

        let point = hilbert::Point::new(0, &raw_point);

        let out = point.hilbert_transform(range.bits_required);
        vec![HilbertOutput {
            idx: out.to_str_radix(10),
        }]
    }
}

fn vector_unpack(input: &[u8]) -> Vec<f64> {
    assert!(
        input.len() % 4 == 0,
        "expected input length to be a multiple of 4"
    );
    let mut output = Vec::with_capacity(input.len() / 4);
    for f in input.chunks_exact(4) {
        let bytes: [u8; 4] = f.try_into().expect("slice with incorrect length");
        output.push(f32::from_le_bytes(bytes) as f64);
    }
    output
}
