import networkx
import pandas as pd
from pyvis.network import Network
import time

path = "output/"

df = pd.read_csv(path + "network.csv")

G = networkx.from_pandas_edgelist(df,
                                  source="Source",
                                  target="Target",
                                  edge_attr="weight")

net = Network(notebook=True, cdn_resources="remote")
l1 = ['input', 'inner', 'output']
col = ['red', 'black', 'green']

for src, dst, data in G.edges(data=True):
    net.add_node(src, title=src, color=col[l1.index(src.split('_')[0])])
    net.add_node(dst, title=dst, color=col[l1.index(dst.split('_')[0])])
    net.add_edge(src, dst, title=data['weight']) 

net.show(f'{path}graphs/{time.strftime("%d.%m.%Y_%H.%M.%S")}.html')
