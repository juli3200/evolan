import json
import os
import seaborn as sns
import matplotlib.pyplot as plt

r"""
path_ = r"C:\Users\julia\Desktop\testp\generations"
output_path = r"C:\Users\julia\Desktop\testp\output"
"""
path_ = input("input path: ")
output_path = input("output path: ")
dim = list(map(lambda x: int(x), input("dim: ").split("x")))
print(type(dim[0]), dim)


class Visual:
    def __init__(self, path, dim):
        self.path = path
        self.cache = [None for _ in os.listdir(path)]
        self.active_generation = 0
        self.dim = dim

    def jump(self, generation):
        if self.cache[generation - 1] is None:
            with open(rf'{self.path}/{generation}.json', 'r') as file:
                data = json.load(file)

            new_data = [[[0 for _ in range(self.dim[0])] for _ in range(self.dim[1])] for _ in range(len(data))]

            for i, step in enumerate(data):
                for type_ in range(1, len(step) + 1):
                    for x, y in step[type_ - 1]:
                        new_data[i][y][x] = type_

            self.cache[generation - 1] = new_data

        self.active_generation = generation

    def save(self, generation):
        self.jump(generation)
        os.mkdir(fr"{output_path}/{generation}")
        for i, step in enumerate(self.cache[self.active_generation - 1]):
            plt.clf()
            sns.set(font_scale=1)
            # plt.figure(figsize=(8, 6))
            sns.heatmap(step, fmt='d', cmap='YlGnBu', cbar=False)
            plt.title('Evolang')
            plt.xlabel('X')
            plt.ylabel('Y')

            plt.savefig(fr"{output_path}/{generation}/{i}.png")
            plt.close("all")


list_ = input('generations').split(',')
V = Visual(path_, dim=dim)

for generation_ in list_:
    V.save(int(generation_))
    print(generation_)
