use::datafusion_playground::helpers::windows::do_something;
use datafusion_playground::helpers::data_frames::{read_sample_df, read_sample_df_batches};


#[tokio::main]
async fn main() {
    let mut x = 5;
    println!("this is x: {}", x);
    x = 6;
    println!("Hello, world!");
    println!("this is x now: {}", x);
    do_something("nothing");

    let batches = read_sample_df_batches().await;
    match batches {
        Ok(good_batch) => { println!("These are the batches: {:?}", good_batch); },
        Err(err) => println!("This is the error: {:?}", err),
    }

    let df = read_sample_df().await;
    match df {
        Ok(good_df) => { println!("This is the df? {:?}", good_df); },
        Err(err) => println!("This is the error: {:?}", err),
    }
}
