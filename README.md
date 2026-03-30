# KoshaDB

KoshaDB is a storage engine using the LSM tree writtent in Rust.

# Current Goal

1. core types + errors + comparator
2. memtable
3. basic DB API: put/get/delete
4. WAL
5. recovery from WAL
6. flush memtable to SSTable
7. read path across memtable + SSTable
8. iterator
9. manifest + metadata tracking
10. multiple SSTables + basic compaction
11. snapshots
12. write batch
13. column families
14. bloom filters
15. block cache + table cache
16. transactions
17. compression
18. background threads
19. backup/debug/read-only mode
20. advanced things like merge operator, prefix iterator, direct I/O
