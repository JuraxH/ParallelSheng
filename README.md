# Parallel Sheng
The purpose of this code is to determine how much faster can the parallel
version of the Sheng DFA be on different sizes of data
scanned. The idea for using the multiple lanes of simd vector to parallelize
the DFA was introduced to me in Data-parallel finite-state machines paper
[[3]](#3) from Microsoft Research and the Sheng DFA by Hyperscan[[1]](#1)[[2]](#2).

## Implementation
The repo contains implementation of two versions of the basic table DFA, where
the successor state look up is performed by indexing array, one version
uses succ[state][symbol] and the other succ[symbol][state].
There is 16 state version of Sheng DFA that uses the `ssse3` `PSHUFB`
instruction for state lookup in masks with successor states for each input
symbol. The Sheng DFA was parallelized using the std rust channels and os threads.

## Running
The Benchmark should be run with --release flag on machine that supports
at least `ssse3`.

## Benchmark
The bytes per nanosecond scanned were measured for each implementation of DFA
when scanning 2^32 bytes, by different chunk sizes starting 2^10 and increasing
by factor of 2. For each experiment the engines were run 10 times and the average
of each engine was used.

## Results
The following table shows the result of benchmark on Intel(R) Core(TM) i5-7500 CPU @ 3.40GHz.

<table border="1">
<thead>
<tr style="text-align: center;">
<th rowspan=2>block size</th>
<th colspan=2>Array[b/ns]</th>
<th colspan=4>Sheng[b/ns]</th>
</tr>
<tr style="text-align: center;">
<th>[state][symbol]</th>
<th>[symbol][state]</th>
<th>1 threads</th>
<th>2 threads</th>
<th>3 threads</th>
<th>4 threads</th>
</tr>
</thead>
<tbody>
<tr style="text-align: center;">
<td>2^10</td>
<td>0.507</td>
<td>0.716</td>
<td>3.132</td>
<td>3.136</td>
<td>3.126</td>
<td>3.123</td>
</tr>
<tr style="text-align: center;">
<td>2^11</td>
<td>0.516</td>
<td>0.721</td>
<td>3.206</td>
<td>0.319</td>
<td>3.176</td>
<td>3.196</td>
</tr>
<tr style="text-align: center;">
<td>2^12</td>
<td>0.515</td>
<td>0.718</td>
<td>3.237</td>
<td>0.62</td>
<td>0.472</td>
<td>0.394</td>
</tr>
<tr style="text-align: center;">
<td>2^13</td>
<td>0.515</td>
<td>0.721</td>
<td>3.24</td>
<td>1.128</td>
<td>0.906</td>
<td>0.725</td>
</tr>
<tr style="text-align: center;">
<td>2^14</td>
<td>0.512</td>
<td>0.717</td>
<td>3.221</td>
<td>1.874</td>
<td>1.81</td>
<td>1.284</td>
</tr>
<tr style="text-align: center;">
<td>2^15</td>
<td>0.514</td>
<td>0.719</td>
<td>3.25</td>
<td>2.7</td>
<td>2.931</td>
<td>2.003</td>
</tr>
<tr style="text-align: center;">
<td>2^16</td>
<td>0.514</td>
<td>0.721</td>
<td>3.258</td>
<td>3.662</td>
<td>3.734</td>
<td>2.87</td>
</tr>
<tr style="text-align: center;">
<td>2^17</td>
<td>0.514</td>
<td>0.719</td>
<td>3.249</td>
<td>4.531</td>
<td>5.548</td>
<td>3.839</td>
</tr>
<tr style="text-align: center;">
<td>2^18</td>
<td>0.513</td>
<td>0.718</td>
<td>3.243</td>
<td>5.072</td>
<td>6.247</td>
<td>4.642</td>
</tr>
<tr style="text-align: center;">
<td>2^19</td>
<td>0.514</td>
<td>0.719</td>
<td>3.251</td>
<td>5.218</td>
<td>6.804</td>
<td>5.344</td>
</tr>
<tr style="text-align: center;">
<td>2^20</td>
<td>0.514</td>
<td>0.718</td>
<td>3.267</td>
<td>5.282</td>
<td>6.594</td>
<td>5.277</td>
</tr>
<tr style="text-align: center;">
<td>2^21</td>
<td>0.514</td>
<td>0.718</td>
<td>3.251</td>
<td>5.339</td>
<td>7.169</td>
<td>5.236</td>
</tr>
<tr style="text-align: center;">
<td>2^22</td>
<td>0.513</td>
<td>0.719</td>
<td>3.251</td>
<td>5.646</td>
<td>7.662</td>
<td>5.561</td>
</tr>
<tr style="text-align: center;">
<td>2^23</td>
<td>0.514</td>
<td>0.716</td>
<td>3.248</td>
<td>5.753</td>
<td>8.282</td>
<td>6.315</td>
</tr>
<tr style="text-align: center;">
<td>2^24</td>
<td>0.514</td>
<td>0.719</td>
<td>3.251</td>
<td>5.854</td>
<td>8.32</td>
<td>6.896</td>
</tr>
<tr style="text-align: center;">
<td>2^25</td>
<td>0.515</td>
<td>0.719</td>
<td>3.258</td>
<td>6.027</td>
<td>8.237</td>
<td>9.99</td>
</tr>
<tr style="text-align: center;">
<td>2^26</td>
<td>0.514</td>
<td>0.721</td>
<td>3.251</td>
<td>5.964</td>
<td>8.644</td>
<td>10.612</td>
</tr>
<tr style="text-align: center;">
<td>2^27</td>
<td>0.514</td>
<td>0.719</td>
<td>3.246</td>
<td>6.038</td>
<td>8.677</td>
<td>10.319</td>
</tr>
<tr style="text-align: center;">
<td>2^28</td>
<td>0.515</td>
<td>0.717</td>
<td>3.259</td>
<td>6.0</td>
<td>8.538</td>
<td>10.071</td>
</tr>
<tr style="text-align: center;">
<td>2^29</td>
<td>0.516</td>
<td>0.719</td>
<td>3.257</td>
<td>6.224</td>
<td>8.718</td>
<td>10.273</td>
</tr>
<tr style="text-align: center;">
<td>2^30</td>
<td>0.515</td>
<td>0.717</td>
<td>3.224</td>
<td>6.339</td>
<td>7.695</td>
<td>10.498</td>
</tr>
</tbody>
</table>

## References
<a id="1">[1]</a>
Xiang Wang and Yang Hong and Harry Chang and KyoungSoo Park and Geoff Langdale and Jiayu Hu and Heqing Zhu [(2019)](https://www.usenix.org/conference/nsdi19/presentation/wang-xiang)
Hyperscan: A Fast Multi-pattern Regex Matcher for Modern CPUs

<a id="2">[2]</a>
Geoff Langdale [(2018)](https://branchfree.org/2018/05/25/say-hello-to-my-little-friend-sheng-a-small-but-fast-deterministic-finite-automaton/)
“Say Hello To My Little Friend”: Sheng, a small but fast Deterministic Finite Automaton

<a id="3">[3]</a>
Mytkowicz, Todd and Musuvathi, Madanlal and Schulte, Wolfram [(2014)](https://doi.org/10.1145/2654822.2541988)
Data-parallel finite-state machines
