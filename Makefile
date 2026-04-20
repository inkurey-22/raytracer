.phony: all debug clean fclean re test

NAME = raytracer

CRATE_DIRS = crates/camera \
			 crates/light \
			 crates/sphere \
			 crates/vec3 \

all:
	cargo build --release
	cp target/release/$(NAME) .

debug: 
	cargo build
	cp target/debug/$(NAME) .

clean:
	cargo clean
	rm -f $(NAME)

fclean: clean

re: fclean all

test:
	@echo "Testing main"
	@cargo test
	@for dir in $(CRATE_DIRS); do \
		echo "Testing $$dir"; \
		cargo test --manifest-path=$$dir/Cargo.toml; \
	done