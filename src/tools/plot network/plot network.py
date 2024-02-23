import networkx
import pandas as pd
from pyvis.network import Network
import time
import sys 
import os

path = sys.argv[1]

if not path.isinstance(str):
    raise TypeError("path must be of type 'str'")


df = pd.read_csv(".cache/network.csv")

G = networkx.from_pandas_edgelist(df,
                                  source="Source",
                                  target="Target",
                                  edge_attr="weight")

net = Network(notebook=True, cdn_resources="remote")
l1 = ['input', 'inner', 'output']
col = ['red', 'black', 'green']

added_nodes = set()  # To keep track of nodes that have been added

for src, dst, data in G.edges(data=True):
    add_edge = False
    if src not in added_nodes:
        net.add_node(src, title=src, color=col[l1.index(src.split('_')[0])])
        added_nodes.add(src)
        add_edge = True
    if dst not in added_nodes:
        net.add_node(dst, title=dst, color=col[l1.index(dst.split('_')[0])])
        added_nodes.add(dst)
        add_edge = True
    
    if add_edge:
        net.add_edge(src, dst, title=data['weight'], arrows='to')

net.show(f'{path}/{time.strftime("%d.%m.%Y_%H.%M.%S")}.html')
