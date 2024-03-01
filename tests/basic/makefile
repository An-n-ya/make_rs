edit : main.o kbd.o command.o display.o \
       insert.o search.o files.o utils.o
	cc -o edit main.o kbd.o command.o display.o \
				insert.o search.o files.o utils.o
main.o : main.c defs.h
	cc -c main.c -o main.o
kbd.o : kbd.c defs.h command.h
	cc -c kbd.c -o kbd.o
command.o : command.c defs.h command.h
	cc -c command.c -o command.o
display.o : display.c defs.h buffer.h
	cc -c display.c -o display.o
insert.o : insert.c defs.h buffer.h
	cc -c insert.c -o insert.o
search.o : search.c defs.h buffer.h
	cc -c search.c -o search.o
files.o : files.c defs.h buffer.h command.h
	cc -c files.c -o files.o
utils.o : utils.c defs.h
	cc -c utils.c -o utils.o
clean :
	rm edit main.o kbd.o command.o display.o \
		insert.o search.o files.o utils.o
