1. Support partitioning databases into multiple column faminilies with all created with the family name "default"
2. Consistent view across families even after crash recovery
3. Support cross-column family operations through API
4. Put: insert a key into the database and if there is aleady available then overwirte
5. Write: allow multiple key-values, update, delete, insert; it guranteess either all are inserted or none are
6. DeleteRange: delete all keys in a range
7. Key and values are byte stream with no limit to the size
8. Get: fetch single key-values from the database
9. MultiGet: fetch multiple key-values from the database with consistent data
10. All data in the database is stored in logically sorted order
11. Iterator: allows range scan on the database, can also be used to do reverse iteration from a point, a consistent point-it-time view is created when the Iterator is created. Keeps a refrence count on all files corresponding to a point-in-time-view of the database
12. Snapshot: allows the database to create a point-in-time view, not persistent over db restart
13. Support multi-operational transactions, both optimistic and pessimistic
14. Key-prefix iterator: enable key-prefix based filtering using the bloom filter, also uses bloom filters to avoid looking into files that do not have the key-prefix 
15. WAL: supports put, delete, merge and are stored in in-memory buffer called memtable 
16. Batch-commit: mechanism to batch transactions into the log using single fsync call
17. Checksum: use checksums to detect corruption
18. Multi-threaded compaction
19. Manifest: a manifest log file is used to keep all the database state changes
20. Avoiding stalls: keep a small set of threads explictliy for flushing memtables to storage
21. Compaction filter: modify the value of the key or drop the key entirely as part of the compaction process
22. Read only mode
23. Databse debug: save the debug logs to log file
24. Data compression
25. BackupEngine
26. Block cache: LRU cache for blocks to serve reads
27. Table cache: caches open file descriptors, the file descriptors are for sstfiles
28. I/O Control: direct i/o
29. Memtables:
    - Pluggable memtable: default implmentation using skiplist, also availale, vector memtable, prefix memtable
    - Garbase collection during memtable flush
30. Merge operator: can combine multiple put and merge records into a single one
31. DB ID: create unique id at the time of database creation and stored in identity file, can also be added to the manifest file
