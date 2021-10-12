use wasi_interface_gen::wasi_interface;

#[wasi_interface]
mod component {
    struct SimpleString {
        s: String,
    }

    pub struct PolarityScores {
        pub compound: f64,
        pub positive: f64,
        pub negative: f64,
        pub neutral: f64,
    }

    fn sentiment(input: SimpleString) -> PolarityScores {
        super::sentiment(&input.s)
    }

    fn sentiment_vec(input: SimpleString) -> Vec<PolarityScores> {
        vec![super::sentiment(&input.s)]
    }
}

fn sentiment(input: &str) -> component::PolarityScores {
    lazy_static::lazy_static! {
        static ref ANALYZER: vader_sentiment::SentimentIntensityAnalyzer<'static> =
            vader_sentiment::SentimentIntensityAnalyzer::new();
    }

    let scores = ANALYZER.polarity_scores(input);
    component::PolarityScores {
        compound: scores["compound"],
        positive: scores["pos"],
        negative: scores["neg"],
        neutral: scores["neu"],
    }
}
