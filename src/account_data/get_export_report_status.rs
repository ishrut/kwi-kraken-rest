use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportReportStatus {
    pub id: String,
    pub descr: String,
    pub format: String,
    pub report: String,
    pub subtype: String,
    pub status: String,
    pub flags: String,
    pub fields: String,
    pub createdtm: String,
    pub expiretm: String,
    pub starttm: String,
    pub completedtm: String,
    pub datastarttm: String,
    pub dataendtm: String,
    pub aclass: String,
    pub asset: String,
}

impl ExportReportStatus {
    /// Warning: function untested, returns
    /// Gets export report status
    pub async fn get(report: &str) -> Result<Vec<Self>, Error> {
        let queries = build_queries!(report ; );
        let response = get_private_json::<Vec<Self>>(EXPORT_STATUS_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
