Bits, bytes, endianness
Primitive encoding (u32, varint, etc.)
Key/value representation
Record format (fixed vs variable length)
Serialization / deserialization
Checksums (CRC, etc.)
Page size definition (e.g., 4KB)
Page structure (raw memory layout)
Page header (id, flags, LSN, etc.)
Slot directory (for variable-length records)
Free space offset management
Fragmentation handling inside page
Record insertion/update/delete in page
Page compaction (defragmentation)
Page ID abstraction (logical vs physical)
File abstraction (heap file / segment file)
File layout (single file vs multiple segments)
Block I/O (pread/pwrite vs mmap)
Direct I/O vs buffered I/O decisions
Page allocation strategy
Free list / free space map
Disk space reclamation
Basic heap/table storage (unsorted)
Sequential scan (full table scan)
Iterator abstraction (forward/backward)
In-memory structures (temporary buffers)
Memtable / write buffer (even for B-tree systems optionally)
Buffer pool (page cache)
Page pin/unpin mechanism
Dirty page tracking
Eviction policy (LRU, clock, etc.)
Read-ahead / prefetching
Write batching / coalescing
Write-Ahead Log (WAL) format
Log record structure (redo/undo)
Log Sequence Number (LSN)
WAL append mechanism
Group commit
Flush ordering (WAL before data)
Checkpointing
Crash recovery (redo phase)
Undo logging (optional depending on design)
Recovery modes (redo-only vs undo/redo)
Log truncation / archiving
B-tree / B+ tree node structure
Internal vs leaf node layout
Key ordering and comparison
Node search algorithm
Node split (insert overflow)
Node merge / redistribution (delete underflow)
Root page handling
Tree height management
Range scan over index
Secondary index structure
Covering index concept
LSM components (if you go that route)
Memtable (skiplist / tree)
Immutable memtable
SSTable format
Block index inside SSTable
Bloom filters
Compaction strategies (leveled, tiered)
Write amplification handling
Read amplification handling
Manifest / version metadata
Transactions (basic begin/commit)
Atomicity guarantees
Isolation levels (read committed, snapshot, etc.)
Concurrency control model
Lock manager (row/page level locks)
Latches vs locks (important distinction)
Deadlock detection / avoidance
MVCC (multi-version concurrency control)
Version chains / visibility rules
Garbage collection of old versions
Schema metadata (tables, indexes)
System catalog storage
Type system basics
Constraints (primary key, unique)
Null handling
Compression (page-level or block-level)
Encoding optimizations (prefix, delta)
Encryption at rest
Checksumming per page/block
Corruption detection and recovery
Background workers (threads/tasks)
Compaction workers (LSM)
Checkpoint workers
Cleaner / vacuum process
Stats collection
Snapshotting (consistent read views)
Backup / restore
Time-travel queries (via MVCC)
Replication primitives (log shipping)
Consensus integration (Raft, etc.)
Distributed storage layout
Sharding / partitioning
Distributed transactions (2PC, etc.)
Configuration system
Metrics and observability
Debugging / inspection tools
Consistency checker (verify structures)
Migration / upgrade handling
