#ifndef RENDER_H
#define RENDER_H

#ifdef __cplusplus
extern "C" {
#endif

unsigned char* render(unsigned char* data, unsigned char* colors, int width, int height);
unsigned char* cpu_render(unsigned char* data, unsigned char* colors, int width, int height);

#ifdef __cplusplus
}
#endif


#endif // RENDER_H
