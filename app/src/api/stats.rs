use axum::Json;
use crate::clients::database::get_stats;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    status: String,
    message: String,
    num_keys: u64,
    num_bytes: u64,
    num_locks: u64,
    num_pending_compactions: u64,
}

pub async fn stats() -> Json<StatsResponse> {
    let stats = get_stats();
    match stats {   
        Ok(stats) => {
            Json(StatsResponse {
                status: "success".to_string(),
                message: "Stats fetched successfully".to_string(),
                num_keys: stats.num_keys,
                num_bytes: stats.num_bytes,
                num_locks: stats.num_locks,
                num_pending_compactions: stats.num_pending_compactions,
            })
        }
        Err(e) => {
            Json(StatsResponse {
                status: "error".to_string(),
                message: format!("Failed to fetch stats: {}", e),
                num_keys: 0,
                num_bytes: 0,
                num_locks: 0,
                num_pending_compactions: 0,
            })
        }
    }
}   

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn test_stats() {
        let response = stats().await;
        println!("Response: {:?}", response);
        assert_eq!(response.status, "success");
        assert!(response.num_keys > 0);
        assert!(response.num_bytes > 0);
        
    }
}   