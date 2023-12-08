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

added_nodes = set()  # To keep track of nodes that have been added

for src, dst, data in G.edges(data=True):
    if src not in added_nodes:
        net.add_node(src, title=src, color=col[l1.index(src.split('_')[0])])
        added_nodes.add(src)
    if dst not in added_nodes:
        net.add_node(dst, title=dst, color=col[l1.index(dst.split('_')[0])])
        added_nodes.add(dst)
    
    net.add_edge(src, dst, title=data['weight'], arrows='to')

net.show(f'{path}graphs/{time.strftime("%d.%m.%Y_%H.%M.%S")}.html')
