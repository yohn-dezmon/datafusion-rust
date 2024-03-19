/*
DataFrame:
https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html

DataFrames are “lazy” in the sense that most methods do not actually compute anything, 
they just build up a plan. Calling collect executes the plan using the same 
DataFusion planning and execution process used to execute SQL and other queries.

The most common way to create a dataframe is from a CSV or Parquet file...


*/
use anyhow::Result;
use datafusion::{arrow::array::RecordBatch, dataframe::DataFrame, execution::{context::SessionContext, options::CsvReadOptions}};
// returns Result<Vec<RecordBatch>, anyhow::Error>
pub async fn read_sample_df_batches() -> Result<Vec<RecordBatch>> {
    let ctx = SessionContext::new();
    let df = ctx.read_csv("src/data/sample_data.csv", CsvReadOptions::new()).await?;
    let batches = df.collect().await?;

    Ok(batches)
}
// returns Result<DataFrame, anyhow::Error>
// you can pass this to a function that applies a transformation to it
// then use collect() to get the RecordBatch vector...
pub async fn read_sample_df() -> Result<DataFrame> {
    let ctx = SessionContext::new();
    let df = ctx.read_csv("src/data/sample_data.csv", CsvReadOptions::new()).await?;
    Ok(df)
}
