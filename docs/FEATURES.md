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
11. Iterator: allows range scan on the database, can also be used to do reverse iteration from a point, a consistent point-it-time view is created when the Iterator is created.
12. Snapshot: allows the database to create a point-in-time view
13.
