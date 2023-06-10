use chrono::{DateTime, Duration, Local};
use tokio::io::{self, AsyncWriteExt};
use up_api::v1::{
    transactions::{ListTransactionsOptions, TransactionResource},
    Client,
};

pub struct UpClient {
    internal_client: Client,
}

impl UpClient {
    pub fn new(auth_token: String) -> UpClient {
        let internal_client = Client::new(auth_token);

        UpClient {
            internal_client: internal_client,
        }
    }

    // TODO: Switch away from returning up_api's TransactionResource here.
    /// Fetch a set of transactions for a period of time.
    pub async fn list_transactions(
        &self,
        since: Option<DateTime<Local>>,
        until: Option<DateTime<Local>>,
    ) -> Vec<TransactionResource> {
        let mut options = ListTransactionsOptions::default();
        options.page_size(100);
        if let Some(since) = since {
            options.filter_since(since.format("%+").to_string());
        }

        if let Some(until) = until {
            options.filter_until(until.format("%+").to_string());
        }
        let mut calls = 1;
        let mut transactions: Vec<TransactionResource> = Vec::new();
        let mut resp = self
            .internal_client
            .list_transactions(&options)
            .await
            .expect("Failed to fetch transactions");

        transactions.append(&mut resp.data);
        calls += 1;
        while let Some(next_resp) = resp.next(&self.internal_client).await {
            resp = next_resp.expect("Failed to fetch transactions page");
            transactions.append(&mut resp.data);
            dbg!(calls);
            calls += 1;
        }
        transactions
    }
}
