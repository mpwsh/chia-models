use std::path::Path;

use reqwest::ClientBuilder;
use reqwest::Response;

use crate::Error;
use crate::util::load_pem_pair;

mod model;
pub use model::BlockchainState;
use model::BlockchainStateResponse;

pub struct Client {
	host: String,
	port: u16,
	http: reqwest::Client
}

impl Client {
	pub async fn new(host: &str, port: u16, key_file: impl AsRef<Path>, cert_file: impl AsRef<Path>) -> Result<Self, Error> {
		let identity = load_pem_pair(key_file, cert_file).await?;
		let http = ClientBuilder::new()
			.danger_accept_invalid_certs(true)
			//.danger_accept_invalid_hostnames(true)
			.identity(identity)
			.build()?;
		Ok(Self{
			host: host.to_string(),
			port: port,
			http: http
		})
	}

	pub async fn get_blockchain_state(&self) -> Result<BlockchainState, Error> {
		let response = self.cmd("get_blockchain_state").await?;
		let response: BlockchainStateResponse = response.json().await?;
		Ok(response.blockchain_state)
	}

	async fn cmd(&self, command: &str) -> Result<Response, reqwest::Error> {
		let url = self.make_url(command);
		self.http.post(url).header("Content-Type", "application/json").body("{}").send().await
	}

	fn make_url(&self, command: &str) -> String {
		format!("https://{}:{}/{}", &self.host, self.port, &command)
	}
}

