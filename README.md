# Rust LSM

A toy implementation of Log-Structured Merge-Tree (LSM tree), which is used in LevelDB and RocksDB.

This is meant for research purpose only, dont use it in production.

Finished:

- [x] Memtable
- [x] Persisted SSTable
- [x] SSTable indexing with skip list
- [ ] Persist SSTable index instead of re-calculate
- [ ] Periodically flush Memtable into SSTable
- [ ] Compaction
- [ ] WAL and recover from crash
- [ ] Thread safe
- [ ] Configurable
