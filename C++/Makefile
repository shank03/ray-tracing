CC := clang++
CFLAGS := -Wall -std=c++17 -O3 -D_GLIBCXX_ASSERTIONS
LIB :=
INC := -I.

SRC_DIR := $(CURDIR)/src
BIN_DIR := $(CURDIR)/bin
BUILD_DIR := $(CURDIR)/build
TARGET := $(BIN_DIR)/ray-tracing

IS_WIN := false
ifeq ($(OS),Windows_NT)
	TARGET := $(BIN_DIR)/ray-tracing.exe
	IS_WIN := true
endif

all: $(BUILD_DIR)/main.o
	$(shell if [ ! -d "$(BIN_DIR)" ]; then\
	    mkdir "$(BIN_DIR)";\
	fi)
	$(CC) $^ -o $(TARGET) $(LIB)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	$(shell if [ ! -d "$(BUILD_DIR)" ]; then\
    	mkdir "$(BUILD_DIR)";\
    fi)
	$(CC) $(CFLAGS) $(INC) -c -o $@ $<

$(BUILD_DIR)/main.o: $(SRC_DIR)/main.cpp

clean:
	rm -rf $(BUILD_DIR)/*.o $(TARGET)
