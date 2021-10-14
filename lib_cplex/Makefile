OUTPUT := modelo.run

FLAGS_COMPILADOR = -O3 -m64 -O -fPIC -fexceptions -DNDEBUG -DIL_STD -std=c++0x -std=c++11 -static-libstdc++ -static-libgcc -Wall

# Deixando aqui pra dá pra compilar em um pc com o cplex instalado n o opt
CAMINHO_INCLUDE = -I/opt/cplex/cplex/include -I/opt/cplex/concert/include
CAMINHO_LIB = -L/opt/cplex/cplex/lib/x86-64_linux/static_pic -L/opt/cplex/concert/lib/x86-64_linux/static_pic
LIBS = -lilocplex -lcplex -lconcert -lm -pthread -ldl

CPLEX_FLAGS = $(CAMINHO_INCLUDE) $(CAMINHO_LIB) $(LIBS) $(FLAGS_COMPILADOR)

CXX = g++

all: $(OUTPUT)

$(OUTPUT): main.o
	$(CXX) -o $(OUTPUT) main.o $(CPLEX_FLAGS)

main.o: main.cpp
	$(CXX) -c main.cpp -o main.o $(CPLEX_FLAGS)

clean:
	@rm -f main.o $(OUTPUT)
