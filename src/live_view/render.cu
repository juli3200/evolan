#include "render.h"
#include <iostream>
#include <stdio.h>

__global__ void render_kernel(unsigned char* data, unsigned char* image, unsigned char* colors) {
    const int index = threadIdx.x + threadIdx.y * blockDim.x;

    // multiply the index by 3 because each pixel has 3 values (r, g, b)
    
    image[index* 3] = colors[data[index]*3];
    image[index* 3 + 1] = colors[data[index] * 3 + 1];
    image[index* 3 + 2] = colors[data[index] * 3 + 2];
    

}



/*
param: data - array of data to render in the form of [0,1,1,2,1,0,0,0,...]]
    where each number represents a color in the colors array

param: colors - array of colors to render in the form of [r, g, b, r, g, b, ...] is limited to 16 colors

*/
unsigned char* render(unsigned char* data, unsigned char* colors, int width, int height){

    // check input
    if (width <= 0 || height <= 0) {
        std::cout << "width and height must be greater than 0" << std::endl;
        return NULL;
    }

    // size of data
    int data_size = sizeof(unsigned char) * width * height;

    // creating an array 3 times bigger then the data array
    // because each pixel has 3 values (r, g, b)
    unsigned char* image = (unsigned char*)malloc(sizeof(unsigned char) * width * height * 3);
    
    std::cout << sizeof(data_size) << std::endl;

    // create pointers to the data and colors arrays gpu
    unsigned char* dev_data;
    unsigned char* dev_colors;
    unsigned char* dev_image;

    // allocate memory on the gpu
    cudaMalloc((void**)&dev_data, data_size);
    cudaMalloc((void**)&dev_colors, sizeof(unsigned char) * 3  * /*colors array is limited to 16 colors*/16);
    cudaMalloc((void**)&dev_image, data_size * 3);

    // copy the data and colors arrays to the gpu
    cudaMemcpy(dev_data, data, data_size, cudaMemcpyHostToDevice);
    cudaMemcpy(dev_colors, colors, sizeof(unsigned char) * 3  * 16, cudaMemcpyHostToDevice);

    dim3 grid(width, height);

    render_kernel<<<1, grid>>>(dev_data, dev_image, dev_colors);

    cudaMemcpy(image, dev_image, data_size*3, cudaMemcpyDeviceToHost);

    cudaFree(dev_data);
    cudaFree(dev_colors);
    cudaFree(dev_image);
    
    return image;
}



unsigned char* cpu_render(unsigned char* data, unsigned char* colors, int width, int height) {
    // Check input
    if (width <= 0 || height <= 0) {
        std::cout << "Width and height must be greater than 0" << std::endl;
        return NULL;
    }

    unsigned char* image = (unsigned char*)malloc(sizeof(unsigned char) * width * height * 3);

    // Render on CPU
    for (int j = 0; j < height; ++j) {
        for (int i = 0; i < width; ++i) {
            const int index = j * width + i;
            image[index * 3] = colors[data[index] * 3];
            image[index * 3 + 1] = colors[data[index] * 3 + 1];
            image[index * 3 + 2] = colors[data[index] * 3 + 2];
        }
    }
    return image;
}