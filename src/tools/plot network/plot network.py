import networkx
import pandas as pd
from pyvis.network import Network
import time
import os

path = "output/"

df = pd.read_csv(path + "network.csv")

G = networkx.from_pandas_edgelist(df,
                                  source="Source",
                                  target="Target",
                                  edge_attr="weight")

net = Network(notebook=True, cdn_resources="remote")

net.from_nx(G)
net.show(f'{path}graphs/{time.strftime("%d.%m.%Y_%H.%M.%S")}.html')



