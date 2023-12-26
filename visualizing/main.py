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

class Visual:
    def __init__(self, path):
        self.path = path
        self.cache = [None for _ in os.listdir(path)]
        self.active_generation = 0

    def jump(self, generation):
        if self.cache[generation] is None:
            with open(rf'{self.path}\{generation}.json', 'r') as file:
                data = json.load(file)

            d = {'Empty': 0, 'Bot': 1}
            for step in range(len(data)):
                for y in range(len(data[step])):
                    for x in range(len(data[step][y])):
                        try:
                            data[step][y][x] = d[data[step][y][x]]
                        except TypeError:
                            data[step][y][x] = 1

            self.cache[generation] = data

        self.active_generation = generation

    def save(self, generation):
        self.jump(generation)
        os.mkdir(fr"{output_path}/{generation}")
        for i, step in enumerate(self.cache[self.active_generation]):
            plt.clf()
            sns.set(font_scale=1)
            #plt.figure(figsize=(8, 6))
            sns.heatmap(step, fmt='d', cmap='YlGnBu', cbar=False)
            plt.title('Evolang')
            plt.xlabel('X')
            plt.ylabel('Y')

            plt.savefig(fr"{output_path}/{generation}/{i}.png")
            plt.close("all")


list_ = input('generations').split(',')
V = Visual(path_)

for generation_ in list_:
    V.save(int(generation_))
    print(generation_)
