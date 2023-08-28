@0x9cf9878fd3dd8473;

interface Publications {
    create @0 (ns :Text, rel :Text, schema :import "./definitions.capnp" .Schema, owner :Data);
    push @1 (ns :Text, rel :Text, tx :import "./definitions.capnp" .Tx, sig :Data);
}
