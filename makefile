# Makefile for live_view project

# Compiler and flags
NVCC = nvcc
CFLAGS = -c
LDFLAGS =

# Directories
SRC_DIR = src/live_view
LIB_DIR = lib

# Source files
SRCS = $(SRC_DIR)/live_view.c $(SRC_DIR)/render.cu

# Object files
OBJS = $(LIB_DIR)/lw.o $(LIB_DIR)/r.o

# Target executable
TARGET = $(LIB_DIR)/live_view

# Build rule for object files
$(LIB_DIR)/%.o: $(SRC_DIR)/%.c
	$(NVCC) $(CFLAGS) $< -o $@

$(LIB_DIR)/%.o: $(SRC_DIR)/%.cu
	$(NVCC) $(CFLAGS) $< -o $@

# Build rule for the target executable
$(TARGET): $(OBJS)
	$(NVCC) $(OBJS) -o $(TARGET) $(LDFLAGS)

# Clean rule
clean_unix:
	rm -f $(LIB_DIR)/*.o $(TARGET)

# Clean rule for Windows
clean_win:
	erase /Q $(LIB_DIR)\*.o
