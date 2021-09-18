build_release: 
	cargo build --release 
run_debug:
	cargo run
run_release:
	./target/release/kitsu
start_service:
	sudo systemctl start kitsu.service 
stop_service:
	sudo systemctl stop kitsu.service 
status_service:
	sudo systemctl status kitsu.service
restart_service:
	sudo systemctl restart kitsu.service 