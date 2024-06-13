# cosmos block stats substreams

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
