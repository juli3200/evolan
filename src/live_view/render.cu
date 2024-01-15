#include "render.h"
#include <iostream>

__global__ void render_kernel(int* a, int* b, int* c) {
    
}

void hello() {
    int a[100];
    int b[100];
    int c[100];

    for (int i = 0; i < 100; i++) {
        a[i] = i;
        b[i] = i;
    }

    int* dev_a;
    int* dev_b;
    int* dev_c;

    cudaMalloc((void**)&dev_a, 100 * sizeof(int));
    cudaMalloc((void**)&dev_b, 100 * sizeof(int));
    cudaMalloc((void**)&dev_c, 100 * sizeof(int));

    cudaMemcpy(dev_a, a, 100 * sizeof(int), cudaMemcpyHostToDevice);
    cudaMemcpy(dev_b, b, 100 * sizeof(int), cudaMemcpyHostToDevice);

    render_kernel<<<1, 100>>>(dev_a, dev_b, dev_c);

    cudaMemcpy(c, dev_c, 100 * sizeof(int), cudaMemcpyDeviceToHost);

    cudaFree(dev_a);
    cudaFree(dev_b);
    cudaFree(dev_c);


    
    std::cout << "Hello from cuda!" << std::endl;
}

/*
param: data - array of data to render in the form of [0,1,1,2,1,0,0,0,...]]
    where each number represents a color in the colors array

param: colors - array of colors to render in the form of [r, g, b, r, g, b, ...]

*/
unsigned char* render(unsigned char* data, unsigned char* colors, int width, int height){
    // creating an array 3 times bigger then the data array
    // because each pixel has 3 values (r, g, b)
    unsigned char* image = new unsigned char[height * width * 3];
    // initializing the image array to 0
    for (int i = 0; i < 100 * 100 * 3; i++) {
        image[i] = 0;
    }

    
    for (int i = 0; i < 100 * 100; i++) {
        int color = data[i];
        image[i * 3] = colors[color * 3];
        image[i * 3 + 1] = colors[color * 3 + 1];
        image[i * 3 + 2] = colors[color * 3 + 2];
    }
    return image;
}