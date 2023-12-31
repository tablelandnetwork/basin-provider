@0x9cf9878fd3dd8473;

interface Publications {
    create @0 (ns :Text, rel :Text, schema :import "./definitions.capnp" .Schema, owner :Data, cache_duration :Int64) -> (exists :Bool);
    push @1 (ns :Text, rel :Text, tx :import "./definitions.capnp" .Tx, sig :Data);
    upload @2 (ns :Text, rel :Text, size: UInt64, timestamp: Int64) -> (callback :Callback);
    interface Callback {
        write @0 (chunk :Data);
        done @1 (sig :Data);
    }

    list @3 (owner :Data) -> (publications :List(Text));
    deals @4 (ns :Text, rel :Text, limit :UInt32, offset :UInt64, before :Int64, after :Int64) -> (deals :List(import "./definitions.capnp" .DealInfo));
    latestDeals @5 (ns :Text, rel :Text, n :UInt32, before :Int64, after :Int64) -> (deals :List(import "./definitions.capnp" .DealInfo));
}
