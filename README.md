# rdb_compact

## 编译

cargo build 或者 cargo build --release

## 作用

打开rocksdb的数据目录，进行rocksdb的manual compaction,可以减少文件数

## 参数

**-d 指定rocksdb的数据目录**

**-t 指定最多的compaction线程数**

**-o 指定最大打开的文件数**

## Example

4核8G的机器最好设为：

**./rdb_compact -d ./data -o 5000 -t 4**
