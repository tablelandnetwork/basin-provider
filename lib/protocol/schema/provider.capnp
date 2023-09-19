@0x9cf9878fd3dd8473;

interface Publications {
    create @0 (ns :Text, rel :Text, schema :import "./definitions.capnp" .Schema, owner :Data) -> (exists :Bool);
    push @1 (ns :Text, rel :Text, tx :import "./definitions.capnp" .Tx, sig :Data);

    upload @2 (ns :Text, rel :Text, size: UInt64) -> (callback :Callback);
    interface Callback {
        write @0 (chunk :Data);
        done @1 (sig :Data);
    }
}
