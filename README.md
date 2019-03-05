# cita_db_insert

cita_db_insert 模拟chain的插入rocksdb

**-p  预先插入多少高度的块**

**-t  预插入后，以多少毫秒的间隔插入**

**-d  rocksdb的目录，默认 ./data**

**-g  memory buget的值，db_write_buffer_size=memory_buget/num_column/2**

例如：

**./cita_db_insert -p 10000000 -t 3000 -d ./test -g 2048**

快速写rocksdb 10000000条消息以后，以3秒的间隔写入，db目录在./test
