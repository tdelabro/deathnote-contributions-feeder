use dotenv::dotenv;
use reqwest::Response;

#[cfg(debug_assertions)]
pub const BACKEND_BASE_URI: &str = "http://localhost:8000";

#[cfg(not(debug_assertions))]
pub const BACKEND_BASE_URI: &str = "http://localhost:80";

pub async fn post(url: String, body: Option<serde_json::Value>) -> Response {
	let client = reqwest::Client::new();
	let mut builder = client.post(url).header("content-type", "application/json");

	if let Some(body) = body {
		builder = builder.body(body.to_string());
	}

	let response = builder.send().await;

	assert!(response.is_ok(), "{}", response.err().unwrap());
	response.unwrap()
}

pub async fn put(url: String, body: Option<serde_json::Value>) -> Response {
	let client = reqwest::Client::new();
	let mut builder = client.put(url).header("content-type", "application/json");

	if let Some(body) = body {
		builder = builder.body(body.to_string());
	}

	let response = builder.send().await;
	assert!(response.is_ok(), "{}", response.err().unwrap());
	response.unwrap()
}

pub async fn delete(url: String) -> Response {
	let client = reqwest::Client::new();
	let builder = client.delete(url);

	let response = builder.send().await;

	assert!(response.is_ok(), "{}", response.err().unwrap());
	response.unwrap()
}

pub async fn get(url: String) -> Response {
	let response = reqwest::get(url).await;

	assert!(response.is_ok(), "{}", response.err().unwrap());
	response.unwrap()
}