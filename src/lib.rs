use std::result::Result;

type TestError<T> = Result<T, Box<dyn std::error::Error>>;

pub fn test() -> TestError<Vec<usize>> {
    Ok(vec![1, 2, 3])
}

fn end_all_unfinished_activities(activities: Vec<u64>) -> TestError<Vec<usize>> {
    activities
            .iter()
            .for_each(|_| {
                match "Ok(true)" {
                    "bla" => {

        

                    },
                    "bli" => {
                        log::warn!(
                            "Activity {} activity ends before it began. That's impossible. Skipping activity.", "test".to_string()
                        )
                    },
                    _ => {}
                };
            });

    Ok(vec![1, 2, 3])
}
