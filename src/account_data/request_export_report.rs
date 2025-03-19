use super::*;

/// struct to request export report
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestExportReport {
    pub id: String,
}

impl RequestExportReport {
    /// Warning: function untested, returns EGeneral: invalid arguments
    /// Requests an export report.
    pub async fn get(
        report: &str,
        description: &str,
        format: Option<&str>,
        fields: Option<Vec<&str>>,
        starttm: Option<&str>,
        endtm: Option<&str>,
    ) -> Result<String, Error> {
        let queries = build_queries!(report, description; format, starttm, endtm);
        let body = match fields {
            Some(val) => format!("{}&fields={}", &queries, &val.join(",")),
            None => queries,
        };
        println!("body: {}", body);
        let response = get_private_json::<String>(ADD_EXPORT_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
