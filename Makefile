.phony: all debug clean fclean test

NAME = raytracer

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

test:
	cargo test