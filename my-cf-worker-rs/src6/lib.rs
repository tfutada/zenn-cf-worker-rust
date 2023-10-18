use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize, Serialize, Debug)]
struct Customer {
    #[serde(rename = "CustomerId")]
    customer_id: u32,
    #[serde(rename = "CompanyName")]
    company_name: String,
    #[serde(rename = "ContactName")]
    contact_name: String,
}

#[event(fetch, respond_with_errors)]
pub async fn main(request: Request, env: Env, ctx: Context) -> Result<Response> {
    let company_name = "Bs Beverages".to_string();
    let d1 = env.d1("customer-db")?;
    let statement = d1.prepare("SELECT * FROM Customers WHERE CompanyName = ?1");
    let query = statement.bind(&[company_name.into()])?;
    let result = query.first::<Customer>(None).await?;
    console_log!("result: {:?}", result);

    Response::empty()
}