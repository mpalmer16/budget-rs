use crate::domain::{Budget, BudgetCreationRequest};

pub struct BudgetClient {
    use_https: bool,
    ip_address: String,
    port: u16,
    api_base: String,
}

impl BudgetClient {
    fn url(&self) -> String {
        if self.use_https {
            format!(
                "https://{}:{}/{}",
                self.ip_address, self.port, self.api_base
            )
        } else {
            format!("http://{}:{}/{}", self.ip_address, self.port, self.api_base)
        }
    }

    pub fn new(ip_address: &str, port: u16, api_base: &str, use_https: bool) -> Self {
        Self {
            ip_address: ip_address.to_string(),
            port,
            api_base: api_base.to_string(),
            use_https,
        }
    }

    pub async fn get_budgets(&self) -> Result<Vec<Budget>, reqwest::Error> {
        println!("using url: {}", self.url());
        let budgets: Vec<Budget> = reqwest::Client::new()
            .get(format!("{}/budgets", self.url()))
            .send()
            .await?
            .json()
            .await?;

        Ok(budgets)
    }

    pub async fn create_budget(
        &self,
        budget: &BudgetCreationRequest,
    ) -> Result<(), reqwest::Error> {
        reqwest::Client::new()
            .post(format!("{}/budgets/add", self.url()))
            .json(budget)
            .send()
            .await?
            .json()
            .await?;

        Ok(())
    }
}
