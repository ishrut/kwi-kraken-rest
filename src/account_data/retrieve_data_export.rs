use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct RetrieveDataExport {
    //actually a binary
    pub report: String,
}

//this actually should download a zip
impl RetrieveDataExport {
    /// Warning: function untested, surely buggy as should download a zip
    /// Cannot test yet as unsuccesfull to request export report.
    pub async fn get(id: &str, _path: &str) -> Result<Self, Error> {
        let queries = build_queries!( id; );
        let resp = get_private_json::<Self>(RETRIEVE_EXPORT_URI, Some(&queries)).await?;
        Ok(resp.result)
    }
}
