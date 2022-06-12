# comp

计算文件差集，交集，并集，去重，排序等。

## 开发背景

为什么单独开发这个工具？linux cat， comm 等命令都可以实现上述功能，
但是本人在用的时候踩坑了，莫名其妙的对一些文件没有效果，可能是因为文件内每一行格式的问题，
比如有隐藏字符，空格，制表，换行等符号影响了，总计导致我计算失误。因此独立使用rust重新开发完成如上功能。



## usage

```
⇒  comp -h
comp 1.0.1
Django
Compute intersection,difference,union set between two files, sort or remove duplicate elem from file

USAGE:
    comp [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    diff         display difference, the result is filepath1 - filepath2
    help         Print this message or the help of the given subcommand(s)
    inter        display intersection between filepath1 and filepath2
    sort-asc     display sorted file elems by asc
    sort-desc    display sorted file elems by desc
    union        display union between filepath1 and filepath2
    uniq         display unique elems from input file
```

### 交集

```shell
# sort, uniq 可选
comp inter a.txt b.txt --sort=asc --uniq=true
```
### 差集
```shell
# sort, uniq 可选
comp diff a.txt b.txt --sort=asc --uniq=true
```
### 并集
```shell
# sort, uniq 可选
comp union a.txt b.txt --sort=asc --uniq=true
```
### 排序
```shell
# sort, uniq 可选
comp sort-asc a.txt --uniq=true
```

### 去重
```shell
# sort 可选
comp uniq a.txt --sort=asc
```
