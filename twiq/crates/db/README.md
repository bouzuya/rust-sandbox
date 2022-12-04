# db

## indexes

### composite index 1

- collection : events
- fields     : requested_at: ASC stream_id: ASC stream_seq: ASC
- query scope: Collection

### composite index 2

- collection : events
- fields     : stream_id: ASC stream_seq: ASC
- query scope: Collection

### single field index

- events.data: disable
