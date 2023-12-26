import json
import os
import threading
import tkinter as tk
import seaborn as sns
import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg

path_ = r"C:\Users\julia\Desktop\testp\generations"
output_path = r"C:\Users\julia\Desktop\testp\output"

class Visual:
    def __init__(self, path, root):
        self.path = path
        self.cache = [None for _ in os.listdir(path)]
        self.active_generation = 0
        self.root = root

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
            sns.set(font_scale=1.2)
            plt.figure(figsize=(8, 6))
            sns.heatmap(step, annot=True, fmt='d', cmap='YlGnBu', cbar=False)
            plt.title('List of Lists of Strings')
            plt.xlabel('X Axis Label')
            plt.ylabel('Y Axis Label')

            plt.savefig(fr"{output_path}/{generation}/{i}.png")


def save(generation):
    t = threading.Thread(target=V.save, args=(generation,))
    t.start()

root = tk.Tk()

entry1 = tk.Entry(root)
entry1.pack()

V = Visual(path_, root)

tk.Button(root, text="save", command=lambda: save(int(entry1.get()))).pack()


root.mainloop()
