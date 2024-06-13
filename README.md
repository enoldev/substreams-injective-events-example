# Injective Events Example

Example shocasing how to use the `filtered_events` Injective Foundational Module to get all the events where `type = wasm`.

### Generate protos
```bash
make protogen
```

### Build substreams
```bash
make build
```

### Run substreams
```bash
substreams run substreams.yaml map_events -e mainnet.injective.streamingfast.io:443 --start-block 64987400 --stop-block +10
```
