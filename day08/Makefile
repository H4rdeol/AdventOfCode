##
## EPITECH PROJECT, 2024
## AdventOfCode [WSL: Ubuntu]
## File description:
## Makefile
##

NAME	=	day08

all:	$(NAME)

$(NAME):	$(SRC)
	@cargo build --release
	@cp target/release/$(NAME) .

clean:
	@cargo clean

fclean:	clean
	@rm -f $(NAME)

re:	fclean all

run:	all
	@./$(NAME)

.PHONY:	all clean fclean re run
