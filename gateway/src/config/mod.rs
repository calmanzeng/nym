// Copyright 2020 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::config::template::config_template;
use config::defaults::{DEFAULT_CLIENT_LISTENING_PORT, DEFAULT_MIX_LISTENING_PORT};
use config::NymConfig;
use network_defaults::mainnet::{NYM_API, NYXD_URL, STATISTICS_SERVICE_DOMAIN_ADDRESS};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use url::Url;
use validator_client::nyxd;

pub mod persistence;
mod template;

pub(crate) const MISSING_VALUE: &str = "MISSING VALUE";

// 'DEBUG'
// where applicable, the below are defined in milliseconds
const DEFAULT_PRESENCE_SENDING_DELAY: Duration = Duration::from_millis(10_000);
const DEFAULT_PACKET_FORWARDING_INITIAL_BACKOFF: Duration = Duration::from_millis(10_000);
const DEFAULT_PACKET_FORWARDING_MAXIMUM_BACKOFF: Duration = Duration::from_millis(300_000);
const DEFAULT_INITIAL_CONNECTION_TIMEOUT: Duration = Duration::from_millis(1_500);
const DEFAULT_MAXIMUM_CONNECTION_BUFFER_SIZE: usize = 128;

const DEFAULT_STORED_MESSAGE_FILENAME_LENGTH: u16 = 16;
const DEFAULT_MESSAGE_RETRIEVAL_LIMIT: i64 = 100;

pub fn missing_string_value() -> String {
    MISSING_VALUE.to_string()
}

fn bind_all_address() -> IpAddr {
    "0.0.0.0".parse().unwrap()
}

fn default_mix_port() -> u16 {
    DEFAULT_MIX_LISTENING_PORT
}

fn default_clients_port() -> u16 {
    DEFAULT_CLIENT_LISTENING_PORT
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Config {
    gateway: Gateway,

    #[serde(default)]
    logging: Logging,
    #[serde(default)]
    debug: Debug,
}

impl NymConfig for Config {
    fn template() -> &'static str {
        config_template()
    }

    fn default_root_directory() -> PathBuf {
        dirs::home_dir()
            .expect("Failed to evaluate $HOME value")
            .join(".nym")
            .join("gateways")
    }

    fn try_default_root_directory() -> Option<PathBuf> {
        dirs::home_dir().map(|path| path.join(".nym").join("gateways"))
    }

    fn root_directory(&self) -> PathBuf {
        self.gateway.nym_root_directory.clone()
    }

    fn config_directory(&self) -> PathBuf {
        self.gateway
            .nym_root_directory
            .join(&self.gateway.id)
            .join("config")
    }

    fn data_directory(&self) -> PathBuf {
        self.gateway
            .nym_root_directory
            .join(&self.gateway.id)
            .join("data")
    }
}

impl Config {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Config::default().with_id(id)
    }

    // builder methods
    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        let id = id.into();
        if self.gateway.private_sphinx_key_file.as_os_str().is_empty() {
            self.gateway.private_sphinx_key_file =
                self::Gateway::default_private_sphinx_key_file(&id);
        }
        if self.gateway.public_sphinx_key_file.as_os_str().is_empty() {
            self.gateway.public_sphinx_key_file =
                self::Gateway::default_public_sphinx_key_file(&id);
        }
        if self
            .gateway
            .private_identity_key_file
            .as_os_str()
            .is_empty()
        {
            self.gateway.private_identity_key_file =
                self::Gateway::default_private_identity_key_file(&id);
        }
        if self.gateway.public_identity_key_file.as_os_str().is_empty() {
            self.gateway.public_identity_key_file =
                self::Gateway::default_public_identity_key_file(&id);
        }

        if self.gateway.persistent_storage.as_os_str().is_empty() {
            self.gateway.persistent_storage = self::Gateway::default_database_path(&id);
        }

        self.gateway.id = id;
        self
    }

    pub fn with_only_coconut_credentials(mut self, only_coconut_credentials: bool) -> Self {
        self.gateway.only_coconut_credentials = only_coconut_credentials;
        self
    }

    pub fn with_enabled_statistics(mut self, enabled_statistics: bool) -> Self {
        self.gateway.enabled_statistics = enabled_statistics;
        self
    }

    pub fn with_custom_statistics_service_url(mut self, statistics_service_url: Url) -> Self {
        self.gateway.statistics_service_url = statistics_service_url;
        self
    }

    pub fn with_custom_nym_apis(mut self, nym_api_urls: Vec<Url>) -> Self {
        self.gateway.nym_api_urls = nym_api_urls;
        self
    }

    pub fn with_custom_validator_nyxd(mut self, validator_nyxd_urls: Vec<Url>) -> Self {
        self.gateway.nyxd_urls = validator_nyxd_urls;
        self
    }

    pub fn with_cosmos_mnemonic(mut self, cosmos_mnemonic: bip39::Mnemonic) -> Self {
        self.gateway.cosmos_mnemonic = cosmos_mnemonic;
        self
    }

    pub fn with_listening_address(mut self, listening_address: IpAddr) -> Self {
        self.gateway.listening_address = listening_address;
        self
    }

    pub fn with_announce_address<S: Into<String>>(mut self, announce_address: S) -> Self {
        self.gateway.announce_address = announce_address.into();
        self
    }

    pub fn with_mix_port(mut self, port: u16) -> Self {
        self.gateway.mix_port = port;
        self
    }

    pub fn with_clients_port(mut self, port: u16) -> Self {
        self.gateway.clients_port = port;
        self
    }

    pub fn announce_host_from_listening_host(mut self) -> Self {
        self.gateway.announce_address = self.gateway.listening_address.to_string();
        self
    }

    pub fn with_custom_persistent_store(mut self, store_dir: PathBuf) -> Self {
        self.gateway.persistent_storage = store_dir;
        self
    }

    pub fn with_custom_version<S: Into<String>>(mut self, version: S) -> Self {
        self.gateway.version = version.into();
        self
    }

    pub fn with_wallet_address(mut self, wallet_address: nyxd::AccountId) -> Self {
        self.gateway.wallet_address = Some(wallet_address);
        self
    }

    // getters
    pub fn get_config_file_save_location(&self) -> PathBuf {
        self.config_directory().join(Self::config_file_name())
    }

    pub fn get_only_coconut_credentials(&self) -> bool {
        self.gateway.only_coconut_credentials
    }

    pub fn get_private_identity_key_file(&self) -> PathBuf {
        self.gateway.private_identity_key_file.clone()
    }

    pub fn get_public_identity_key_file(&self) -> PathBuf {
        self.gateway.public_identity_key_file.clone()
    }

    pub fn get_private_sphinx_key_file(&self) -> PathBuf {
        self.gateway.private_sphinx_key_file.clone()
    }

    pub fn get_public_sphinx_key_file(&self) -> PathBuf {
        self.gateway.public_sphinx_key_file.clone()
    }

    pub fn get_enabled_statistics(&self) -> bool {
        self.gateway.enabled_statistics
    }

    pub fn get_statistics_service_url(&self) -> Url {
        self.gateway.statistics_service_url.clone()
    }

    pub fn get_nym_api_endpoints(&self) -> Vec<Url> {
        self.gateway.nym_api_urls.clone()
    }

    pub fn get_nyxd_urls(&self) -> Vec<Url> {
        self.gateway.nyxd_urls.clone()
    }

    pub fn get_cosmos_mnemonic(&self) -> bip39::Mnemonic {
        self.gateway.cosmos_mnemonic.clone()
    }

    pub fn get_listening_address(&self) -> IpAddr {
        self.gateway.listening_address
    }

    pub fn get_announce_address(&self) -> String {
        self.gateway.announce_address.clone()
    }

    pub fn get_mix_port(&self) -> u16 {
        self.gateway.mix_port
    }

    pub fn get_clients_port(&self) -> u16 {
        self.gateway.clients_port
    }

    pub fn get_persistent_store_path(&self) -> PathBuf {
        self.gateway.persistent_storage.clone()
    }

    pub fn get_packet_forwarding_initial_backoff(&self) -> Duration {
        self.debug.packet_forwarding_initial_backoff
    }

    pub fn get_packet_forwarding_maximum_backoff(&self) -> Duration {
        self.debug.packet_forwarding_maximum_backoff
    }

    pub fn get_initial_connection_timeout(&self) -> Duration {
        self.debug.initial_connection_timeout
    }

    pub fn get_maximum_connection_buffer_size(&self) -> usize {
        self.debug.maximum_connection_buffer_size
    }

    pub fn get_use_legacy_sphinx_framing(&self) -> bool {
        self.debug.use_legacy_framed_packet_version
    }

    pub fn get_message_retrieval_limit(&self) -> i64 {
        self.debug.message_retrieval_limit
    }

    pub fn get_version(&self) -> &str {
        &self.gateway.version
    }

    pub fn get_wallet_address(&self) -> Option<nyxd::AccountId> {
        self.gateway.wallet_address.clone()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Gateway {
    /// Version of the gateway for which this configuration was created.
    #[serde(default = "missing_string_value")]
    version: String,

    /// ID specifies the human readable ID of this particular gateway.
    id: String,

    /// Indicates whether this gateway is accepting only coconut credentials for accessing the
    /// the mixnet, or if it also accepts non-paying clients
    #[serde(default)]
    only_coconut_credentials: bool,

    /// Address to which this mixnode will bind to and will be listening for packets.
    #[serde(default = "bind_all_address")]
    listening_address: IpAddr,

    /// Optional address announced to the validator for the clients to connect to.
    /// It is useful, say, in NAT scenarios or wanting to more easily update actual IP address
    /// later on by using name resolvable with a DNS query, such as `nymtech.net`.
    #[serde(default = "missing_string_value")]
    announce_address: String,

    /// Port used for listening for all mixnet traffic.
    /// (default: 1789)
    #[serde(default = "default_mix_port")]
    mix_port: u16,

    /// Port used for listening for all client-related traffic.
    /// (default: 9000)
    #[serde(default = "default_clients_port")]
    clients_port: u16,

    /// Path to file containing private identity key.
    private_identity_key_file: PathBuf,

    /// Path to file containing public identity key.
    public_identity_key_file: PathBuf,

    /// Path to file containing private sphinx key.
    private_sphinx_key_file: PathBuf,

    /// Path to file containing public sphinx key.
    public_sphinx_key_file: PathBuf,

    /// Whether gateway collects and sends anonymized statistics
    enabled_statistics: bool,

    /// Domain address of the statistics service
    statistics_service_url: Url,

    /// Addresses to APIs from which the node gets the view of the network.
    #[serde(alias = "validator_api_urls")]
    nym_api_urls: Vec<Url>,

    /// Addresses to validators which the node uses to check for double spending of ERC20 tokens.
    #[serde(alias = "validator_nymd_urls")]
    nyxd_urls: Vec<Url>,

    /// Mnemonic of a cosmos wallet used in checking for double spending.
    cosmos_mnemonic: bip39::Mnemonic,

    /// nym_home_directory specifies absolute path to the home nym gateways directory.
    /// It is expected to use default value and hence .toml file should not redefine this field.
    nym_root_directory: PathBuf,

    /// Path to sqlite database containing all persistent data: messages for offline clients,
    /// derived shared keys and available client bandwidths.
    persistent_storage: PathBuf,

    /// The Cosmos wallet address that will control this gateway
    // the only reason this is an Option is because of the lack of existence of a sane default value
    wallet_address: Option<nyxd::AccountId>,
}

impl Gateway {
    fn default_private_sphinx_key_file(id: &str) -> PathBuf {
        Config::default_data_directory(id).join("private_sphinx.pem")
    }

    fn default_public_sphinx_key_file(id: &str) -> PathBuf {
        Config::default_data_directory(id).join("public_sphinx.pem")
    }

    fn default_private_identity_key_file(id: &str) -> PathBuf {
        Config::default_data_directory(id).join("private_identity.pem")
    }

    fn default_public_identity_key_file(id: &str) -> PathBuf {
        Config::default_data_directory(id).join("public_identity.pem")
    }

    fn default_database_path(id: &str) -> PathBuf {
        Config::default_data_directory(id).join("db.sqlite")
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Gateway {
            version: env!("CARGO_PKG_VERSION").to_string(),
            id: "".to_string(),
            only_coconut_credentials: false,
            listening_address: bind_all_address(),
            announce_address: "127.0.0.1".to_string(),
            mix_port: DEFAULT_MIX_LISTENING_PORT,
            clients_port: DEFAULT_CLIENT_LISTENING_PORT,
            private_identity_key_file: Default::default(),
            public_identity_key_file: Default::default(),
            private_sphinx_key_file: Default::default(),
            public_sphinx_key_file: Default::default(),
            enabled_statistics: false,
            statistics_service_url: Url::from_str(STATISTICS_SERVICE_DOMAIN_ADDRESS)
                .expect("Invalid default statistics service URL"),
            nym_api_urls: vec![Url::from_str(NYM_API).expect("Invalid default API URL")],
            nyxd_urls: vec![Url::from_str(NYXD_URL).expect("Invalid default nyxd URL")],
            cosmos_mnemonic: bip39::Mnemonic::generate(24).unwrap(),
            nym_root_directory: Config::default_root_directory(),
            persistent_storage: Default::default(),
            wallet_address: None,
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Logging {}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
struct Debug {
    /// Initial value of an exponential backoff to reconnect to dropped TCP connection when
    /// forwarding sphinx packets.
    #[serde(with = "humantime_serde")]
    packet_forwarding_initial_backoff: Duration,

    /// Maximum value of an exponential backoff to reconnect to dropped TCP connection when
    /// forwarding sphinx packets.
    #[serde(with = "humantime_serde")]
    packet_forwarding_maximum_backoff: Duration,

    /// Timeout for establishing initial connection when trying to forward a sphinx packet.
    #[serde(with = "humantime_serde")]
    initial_connection_timeout: Duration,

    /// Maximum number of packets that can be stored waiting to get sent to a particular connection.
    maximum_connection_buffer_size: usize,

    /// Delay between each subsequent presence data being sent.
    #[serde(with = "humantime_serde")]
    presence_sending_delay: Duration,

    /// Length of filenames for new client messages.
    stored_messages_filename_length: u16,

    /// Number of messages from offline client that can be pulled at once from the storage.
    message_retrieval_limit: i64,

    /// Specifies whether the mixnode should be using the legacy framing for the sphinx packets.
    // it's set to true by default. The reason for that decision is to preserve compatibility with the
    // existing nodes whilst everyone else is upgrading and getting the code for handling the new field.
    // It shall be disabled in the subsequent releases.
    use_legacy_framed_packet_version: bool,
}

impl Default for Debug {
    fn default() -> Self {
        Debug {
            packet_forwarding_initial_backoff: DEFAULT_PACKET_FORWARDING_INITIAL_BACKOFF,
            packet_forwarding_maximum_backoff: DEFAULT_PACKET_FORWARDING_MAXIMUM_BACKOFF,
            initial_connection_timeout: DEFAULT_INITIAL_CONNECTION_TIMEOUT,
            presence_sending_delay: DEFAULT_PRESENCE_SENDING_DELAY,
            maximum_connection_buffer_size: DEFAULT_MAXIMUM_CONNECTION_BUFFER_SIZE,
            stored_messages_filename_length: DEFAULT_STORED_MESSAGE_FILENAME_LENGTH,
            message_retrieval_limit: DEFAULT_MESSAGE_RETRIEVAL_LIMIT,
            // TODO: remember to change it in one of future releases!!
            use_legacy_framed_packet_version: true,
        }
    }
}
