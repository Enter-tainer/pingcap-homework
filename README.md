# top-100-url

## 使用

首先编译代码

```
cargo build --release
```

在 `target/release` 下可以得到编译好的二进制文件 `top-100-url`.

该文件仅接受一个参数, 是将要分析的文件的路径.

分析完成后, 会将结果输出到标准输出中.

## 数据生成

代码见: `url-gen.cxx`

1. 随机生成只含字母和数字的字符串, 长度满足参数为 300, 50 的正态分布.
2. 为了模拟 URL 重复出现的情况, 以 1% 的概率将某个字符串重复输出 x 次, x ~ N(200, 1000), 注意当 x < 0 时不会进行重复输出.
3. 当文件达到指定大小时停止输出.

## 测试结果

```
❯ hyperfine -w 1 -m 3 ./top-100-url
Benchmark #1: ./top-100-url
  Time (mean ± σ):     29.742 s ±  2.268 s    [User: 8.770 s, System: 16.609 s]
  Range (min … max):   27.634 s … 32.142 s    3 runs
```
## 解题思路

见 `writeup.md`
