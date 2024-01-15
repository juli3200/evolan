NVCC = nvcc

LIB_DIR = lib

all: $(LIB_DIR)/render_img.exe

$(LIB_DIR)/live_view.o: src/live_view/live_view.c src/live_view/render.h
	$(NVCC) -c $< -o $@

$(LIB_DIR)/render_cu.o: src/live_view/render.cu src/live_view/render.h
	$(NVCC)  -c $< -o $@

$(LIB_DIR)/render_img.exe: $(LIB_DIR)/live_view.o $(LIB_DIR)/render_cu.o
	$(NVCC)  -o $@ $^


clean:
	rm $(LIB_DIR)/*.o
