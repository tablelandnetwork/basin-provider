@0x8c49da2775b6e7db;

struct Tx {
    commitLSN @0 :UInt64;
    records @1 :List(Record);

    struct Record {
        action @0 :Text;
        timestamp @1 :Text;
        schema @2 :Text;
        table @3 :Text;
        columns @4 :List(Column);
        primaryKey @5 :List(PrimaryKey);

        struct Column {
            name @0 :Text;
            type  @1 :Text;
            value @2 :Data;
        }

        struct PrimaryKey {
            name @0 :Text;
            type  @1 :Text;
        }
    }
}

struct Schema {
    columns @0 :List(Column);

    struct Column {
        name @0 :Text;
        type  @1 :Text;
        isNullable @2 :Bool;
        isPartOfPrimaryKey @3 :Bool;
    }
}

struct DealInfo {
    cid @0 :Text;
    size @1 :UInt32;
    created @2 :Text; 
    isPermament @3 :Bool;
}