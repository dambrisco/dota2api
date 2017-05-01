resolve_vanity_url:
	RUST_BACKTRACE=1 cargo run --example resolve_vanity_url gabelogannewell

get_match_history:
	RUST_BACKTRACE=1 cargo run --example get_match_history

.PHONY: resolve_vanity_url get_match_history
