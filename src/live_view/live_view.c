#include <stdio.h>
#include "render.h"
#include <time.h>


int main() {
    printf("Hello, World!\n");
    unsigned char *data = (unsigned char*)malloc(6000);
    for (int i = 0; i < 6000; i++) {
        data[i] = i % 3;
    }

    unsigned char colors[9] = {255,1,2,4,255,6,8,9,255};

    unsigned char* pcolors = colors;

    // Measure time for render function
    clock_t start_time = clock();
    unsigned char* image = render(data, pcolors, 30, 200);
    clock_t end_time = clock();
    double render_time = ((double) (end_time - start_time)) / CLOCKS_PER_SEC;

    // Print the rendered image
    printf("\nRendered Image:\n");
    for (int i = 0; i < 6 * 3; i++) {
        printf("%i ", image[i]);
    }

    // Print the time taken by the render function
    printf("\nTime taken by render: %f seconds\n", render_time);

    // Measure time for cpu_render function
    start_time = clock();
;  // Assuming the size of the output image is known
    unsigned char* cpu_image = cpu_render(data, pcolors, 30, 200);
    end_time = clock();
    double cpu_render_time = ((double) (end_time - start_time)) / CLOCKS_PER_SEC;

    // Print the time taken by the cpu_render function
    printf("Time taken by cpu_render: %f seconds\n", cpu_render_time);

    return 0;


    return 0;
}

