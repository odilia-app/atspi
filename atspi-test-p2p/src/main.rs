use atspi::{
	connection::{AccessibilityConnection, Peer, P2P},
	proxy::accessible::ObjectRefExt,
	zbus::names::OwnedBusName,
};
use std::{
	sync::{Arc, Mutex},
	time::Duration,
};
use tracing::{info, warn};
use tracing_subscriber::fmt;

const CHILD_NAME: &str = "gedit";
const VERBOSITY: bool = false;
const SLEEP_DURATION: Duration = Duration::from_millis(250); // 0.25s

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// Configure a custom tracing event formatter
	let format = fmt::format()
		.with_level(true) // level: severity level (error, warn, info)
		.with_target(false) // target: originating process::path
		.with_thread_ids(true)
		.with_thread_names(true)
		.compact();

	// Create a `fmt` subscriber that uses our custom event format, and set it as the default.
	tracing_subscriber::fmt().event_format(format).init();

	info!("CI(p2p): Set session accessibility to true");
	atspi::connection::set_session_accessibility(true)
		.await
		.expect("Failed to set session accessibility");

	info!("CI(p2p): Create accessibility connection");
	let a11y = AccessibilityConnection::new()
		.await
		.expect("Failed to create accessibility connection");

	let peers = a11y.peers();

	info!("CI(p2p): Launching child process \"{CHILD_NAME}\"");
	let mut child_process = launch_child(CHILD_NAME, None, VERBOSITY);

	// Registry may need a bit of time to populate with the new app
	tokio::time::sleep(SLEEP_DURATION).await;

	let mapping = bus_names_to_human_readable(&a11y).await;
	print_peers(peers.clone(), &mapping).await;

	// Assert that the second app is part of the makking
	assert!(
		mapping
			.iter()
			.any(|(_bus_name, human_readable_name)| human_readable_name
				.to_lowercase()
				.contains(CHILD_NAME)),
		"App \"{CHILD_NAME}\" not registered as P2P application in Peers list."
	);

	info!("CI(p2p): ✅ Peer insertion assertion passed");
	tokio::time::sleep(SLEEP_DURATION).await;
	info!("CI(p2p): Terminating \"{CHILD_NAME}\"");

	// Termination and removal of child
	child_process.kill().expect("Failed to kill child process");
	child_process.wait().expect("Failed to wait on process");
	tokio::time::sleep(SLEEP_DURATION).await;

	let mapping = bus_names_to_human_readable(&a11y).await;
	print_peers(peers, &mapping).await;

	// Assert that the app is no longer part of the makking
	assert!(
		mapping
			.iter()
			.find(|(_bus_name, human_readable_name)| human_readable_name
				.to_lowercase()
				.contains(CHILD_NAME))
			.is_none(),
		"App \"{CHILD_NAME}\" not removed as P2P application from Peers list."
	);

	info!("CI(p2p): ✅ Peer removal assertion passed");
	info!("CI(p2p): ✅ All assertions passed, exiting");
	Ok(())
}

async fn print_peers(peers: Arc<Mutex<Vec<Peer>>>, mapping: &[(OwnedBusName, String)]) {
	info!("CI(p2p): Printing peers...");

	let peers_locked = match peers.lock() {
		Ok(peers_locked) => peers_locked,
		Err(e) => {
			warn!("Failed to lock peers: {}", e);
			return;
		}
	};
	let total_peers = peers_locked.len();

	// The take iterator adaptor will take max. N elements from left to right, so we reverse the iterator first
	let last_peers: Vec<&Peer> = peers_locked.iter().rev().take(3).collect();

	// If there are more than 3 peers, indicate there are more
	if total_peers > 3 {
		println!("Peer: ... (total: {total_peers})");
	}

	// Print in reverse to restore chronological order
	for peer in last_peers.iter().rev() {
		let unique_name = peer.unique_name();

		// Look up the human-readable name from the mapping
		// Mapping may be longer than `last_peers`
		let human_readable = mapping
			.iter()
			.find_map(|(bus_name, name)| {
				if bus_name.as_str() == unique_name.as_str() {
					Some(name.clone())
				} else {
					None
				}
			})
			.unwrap_or_else(|| "not found".to_string());

		println!("Peer: \"{unique_name}\", human readable name: \"{human_readable}\"");
	}

	if total_peers == 0 {
		info!("CI(p2p): No peers found");
	} else {
		println!();
	}
}

// Gets the accessible apps in registry, returns a mapping of bus names to human readable names
async fn bus_names_to_human_readable(
	a11y: &AccessibilityConnection,
) -> Vec<(OwnedBusName, String)> {
	let conn = a11y.connection();

	let registry_accessible = a11y
		.root_accessible_on_registry()
		.await
		.expect("Failed to get root accessible on registry");

	let children = registry_accessible
		.get_children()
		.await
		.expect("Failed to get children of root accessible");

	// Create a mapping of bus_names to human readable names
	let mut bus_name_to_human_readable: Vec<(OwnedBusName, String)> =
		Vec::with_capacity(children.len());

	for child in children {
		let ap = child
			.as_accessible_proxy(conn)
			.await
			.expect("Failed to get accessible proxy");

		let natural_name = ap.name().await.expect("Failed to get name");
		let bus_name: OwnedBusName = ap.inner().destination().to_owned().into();
		bus_name_to_human_readable.push((bus_name, natural_name));
	}

	bus_name_to_human_readable
}

fn launch_child(child_name: &str, child_arg: Option<&str>, verbose: bool) -> std::process::Child {
	let mut command = std::process::Command::new(child_name);
	if let Some(arg) = child_arg {
		command.arg(arg);
	}

	// With inherit() - child output mixes with parent output
	// With null() - only parent output appears, child is silenced.
	if verbose {
		command
			.stdout(std::process::Stdio::inherit())
			.stderr(std::process::Stdio::inherit());
	} else {
		command
			.stdout(std::process::Stdio::null())
			.stderr(std::process::Stdio::null());
	}

	command.spawn().expect("Failed to launch child process")
}
