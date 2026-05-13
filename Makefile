.phony: all debug clean fclean re test

NAME = raytracer

CRATE_DIRS = crates/vec3 \
			 crates/orientation \
			 crates/color \
			 crates/camera \
			 crates/objects/object_interface \
			 crates/objects/sphere \
			 crates/objects/plane \
			 crates/lights/light_interface \
			 crates/lights/omni_light \
			 crates/lights/directional_light \

$(NAME): all

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
