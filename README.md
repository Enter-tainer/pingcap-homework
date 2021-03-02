# top-100-url

## Idea

1. 读取整个文件, 按 hash 结果将 url append 到不同的文件里面, 保证相同url分布在相同的文件里面
2. 依次处理每一个分片, 统计每个分片里面的前100名 (usize, string), 
    - 具体操作大概是整一个hashmap, 存一下出现次数
    - 然后弄一个大小是100的堆, 把hashmap过一遍.(似乎也可以把哈希表里面的东西都摊平放进数组里面, 然后跑快速选择,是O(n)的, 但是内存占用可能会不够用). 
3. 在处理分片的过程中, 统计总的前100名, 最终获得总的前 100 名.(似乎还是可以用堆搞, 维护一个小顶堆, 和堆顶比较一下就可). 

如果单个分片过大应该如何处理?

1. 处理分片时应当边读边统计, 一次读入一大块然后塞进哈希表, 然后再继续读入是不错的选择. 哈希表的大小不会超过分片本身的大小.

2. 似乎可以直接用 BufReader, with_capacity (read_buffer_size).

如何保证 1G 的内存限制?

1. 预处理阶段, 峰值内存占用大小约为 read_buffer_size.
2. 处理分片时, 内存占用大小约为 read_buffer_size + hashmap_size, 如果使用了快速选择, 内存占用会增加到 vector_size, 大概和哈希表的大小差不多.
3. hashmap_size 大小不定, 与分片中本质不同的 URL 数量有关(O(n)).
4. 处理分片时, 最坏情况为: 分片中每个 url 都不同, 此时, 内存占用大小会约等于分片大小.
5. 不妨假设不存在毒瘤情况.. 常规意义下的访问log应该遵循2-8定律.

总结: 分片大小应当约等于内存限制, 或比内存限制稍小一点, 是比较合理的

## LOG

初版(a090c8b4a0615625f9073f293f4c842f5200de4b)测试 10G 的文件的结果:
```
        Command being timed: "./top-100-url ./urls.10G.txt"
        User time (seconds): 11.37
        System time (seconds): 19.66
        Percent of CPU this job got: 72%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:42.59
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 2541484
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 1
        Minor (reclaiming a frame) page faults: 1236581
        Voluntary context switches: 89658
        Involuntary context switches: 3970
        Swaps: 0
        File system inputs: 12724136
        File system outputs: 19599432
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
```
内存超了, 使用 heaptrack 进行分析. 应该是写文件的缓存开的太大了. 

改了(80d060cf93cf8cf9fc92c32ccaf83b9931ca4bbd), 为写缓存设置全局最大值. 调整后峰值内存占用为 800M, 恰为读写缓存的大小之和.

测试结果如下

```
        Command being timed: "./top-100-url ./urls.10G.txt"
        User time (seconds): 10.07
        System time (seconds): 17.13
        Percent of CPU this job got: 89%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:30.40
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 783676
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 3
        Minor (reclaiming a frame) page faults: 1081449
        Voluntary context switches: 86108
        Involuntary context switches: 2866
        Swaps: 0
        File system inputs: 14294144
        File system outputs: 19601184
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
```
