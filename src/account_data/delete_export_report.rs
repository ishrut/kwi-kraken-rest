use super::*;

/// struct to request delete export report
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteExportReport {
    pub delete: bool,
}

impl DeleteExportReport {
    /// Warning: function untested
    /// Delete export report
    pub async fn get(id: &str, report_type: &str) -> Result<Self, Error> {
        let queries = format!("id={}&type={}", id, report_type);
        let response = get_private_json::<Self>(REMOVE_EXPORT_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
