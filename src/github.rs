use anyhow::{anyhow, Result};
use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

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
        // If 12 hours have passed:
        if SystemTime::now().duration_since(self.last_check).unwrap_or_default() >= Duration::from_secs(43200) {
            self.last_check = SystemTime::now();
            match Client::builder()
                .user_agent("AstaSiteUpdater/0.1.0")
                .build().unwrap()
                .get(GITHUB_ASTA_COMMIT_API)
                .send().await
            {
                Ok(response) => {
                    let nested: Vec<NestedCommitInfo> = response.json().await?;
                    self.commits = nested.into_iter().map(NestedCommitInfo::flatten).collect();
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
struct NestedCommitInfo {
    sha: String,
    commit: Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Commit {
    message: String,
    author: Author,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Author {
    date: String,
}

impl NestedCommitInfo {
    /// Also converts the time to a usable unix timestamp
    fn flatten(self) -> CommitInfo {
        CommitInfo {
            sha: self.sha,
            message: self.commit.message,
            date: DateTime::parse_from_rfc3339(&self.commit.author.date).unwrap().timestamp(),
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
