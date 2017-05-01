resolve_vanity_url:
	RUST_BACKTRACE=1 cargo run --example resolve_vanity_url dannyderanged

get_match_history:
	RUST_BACKTRACE=1 cargo run --example get_match_history dannyderanged

.PHONY: resolve_vanity_url get_match_history
