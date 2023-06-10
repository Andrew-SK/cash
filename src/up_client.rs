use chrono::{DateTime, Duration, Local};
use up_api::v1::{
    transactions::{ListTransactionsOptions, TransactionResource},
    Client,
};

pub struct UpClient {
    internal_client: Client,
}

impl UpClient {
    pub fn new(auth_token: String) -> UpClient {
        dbg!("token: {}", auth_token.clone());
        let internal_client = Client::new(auth_token);

        let new_client = UpClient {
            internal_client: internal_client,
        };

        new_client
    }

    pub async fn list_transactions(
        &self,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Vec<TransactionResource> {
        // If no start is passed default to the start of time itself.
        let realised_start = start.unwrap_or_else(|| {
            let min_dt: DateTime<Local> = DateTime::<Local>::MIN_UTC.into();
            min_dt
        });

        // If no end is passed default to 5 minutes into the future.
        let realised_end = end.unwrap_or_else(|| Local::now() + Duration::minutes(5));

        let mut options = ListTransactionsOptions::default();
        options.filter_since(realised_start.format("%+").to_string());
        options.filter_until(realised_end.format("%+").to_string());

        let res = self
            .internal_client
            .list_transactions(&options)
            .await
            .expect("Failed to fetch transactions");

        res.data
    }
}
