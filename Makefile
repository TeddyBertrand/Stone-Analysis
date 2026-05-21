NAME    = stone_analysis
BIN_PATH    =   target/release/$(NAME)

all:
	cargo build --release
	cp $(BIN_PATH) .

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re