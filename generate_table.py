#!/usr/bin/env python3
from numpy import log2
import pandas as pd

df = pd.read_csv('result.csv', sep=';')

print('<table border="1">')
print('<thead>')
print('<tr style="text-align: center;">')
print('<th rowspan=2>block size</th>')
print('<th colspan=2>Array[b/ns]</th>')
print('<th colspan=4>Sheng[b/ns]</th>')
print('</tr>')
print('<tr style="text-align: center;">')
print('<th>[state][symbol]</th>')
print('<th>[symbol][state]</th>')
print('<th>1 threads</th>')
print('<th>2 threads</th>')
print('<th>3 threads</th>')
print('<th>4 threads</th>')
print('</tr>')
print('</thead>')

print('<tbody>')
for _, row in df.iterrows():
    print('<tr style="text-align: center;">')
    print(f'<td>2^{int(log2(row["block_size"]))}</td>')
    print(f'<td>{row["Table1_bytes_per_ns"]}</td>')
    print(f'<td>{row["Table2_bytes_per_ns"]}</td>')
    print(f'<td>{row["Sheng1_bytes_per_ns"]}</td>')
    print(f'<td>{row["Sheng2_bytes_per_ns"]}</td>')
    print(f'<td>{row["Sheng3_bytes_per_ns"]}</td>')
    print(f'<td>{row["Sheng4_bytes_per_ns"]}</td>')
    print('</tr>')

print('</tbody>')
print('</table>')
