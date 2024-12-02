use anyhow::{anyhow, Result};
use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{Duration, SystemTime};
use thiserror::Error;

const GITHUB_ASTA_COMMIT_API: &str = "https://api.github.com/repos/OctoRocket/Website/commits?per_page=5";

pub struct AstaCommitApi {
    last_check: SystemTime,
    commits: Vec<CommitInfo>,
}

impl AstaCommitApi {
    pub fn new() -> Self {
        Self {
            last_check: SystemTime::UNIX_EPOCH,
            commits: vec![],
        }
    }

    pub async fn get(&mut self) -> Result<Vec<CommitInfo>> {
        // If 12 hours have passed, refresh cached response, else returned cache
        if SystemTime::now().duration_since(self.last_check).unwrap_or_default() >= Duration::from_secs(43200) {
            self.last_check = SystemTime::now();
            match Client::builder()
                .user_agent("AstaSiteUpdater/0.1.0")
                .build().unwrap()
                .get(GITHUB_ASTA_COMMIT_API)
                .send().await
            {
                Ok(response) => {
                    let commit_list = response.json().await?;
                    self.commits = CommitInfo::list_from(commit_list)?;
                    Ok(self.commits.clone())
                },
                Err(e) => Err(e),
            }.map_err(|e| anyhow!(e.to_string()))
        } else {
            Ok(self.commits.clone())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    sha: String,
    message: String,
    // In unix time stamp seconds
    date: i64,
}

impl CommitInfo {
    fn new(v: &Value) -> Result<Self> {
        let data: (String, String, &str) = (|| {
            Some((
                v.get("sha")?.as_str()?.to_string(),
                v.get("commit")?.get("message")?.as_str()?.to_string(),
                v.get("commit")?.get("author")?.get("date")?.as_str()?
            ))
        })().ok_or(GitHubApiError::MalformedJson)?;

        Ok(CommitInfo {
            sha: data.0,
            message: data.1,
            date: DateTime::parse_from_rfc3339(data.2)?.timestamp()
        })
    }

    fn list_from(v: Value) -> Result<Vec<Self>> {
        if !v.is_array() {
            Err(GitHubApiError::MalformedJson.into())
        } else {
            v.as_array().unwrap().iter().map(Self::new).collect()
        }
    }
}

#[derive(Error, Debug, Clone, Copy)]
enum GitHubApiError {
    #[error("Recieved JSON is in an unfamiliar format!")]
    MalformedJson
}
