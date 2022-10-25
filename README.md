![lint workflow](https://github.com/helium/proto/actions/workflows/lint.yml/badge.svg)
![rust workflow](https://github.com/helium/proto/actions/workflows/rust.yml/badge.svg)
![node.js workflow](https://github.com/helium/proto/actions/workflows/node.yml/badge.svg)
![erlang workflow](https://github.com/helium/proto/actions/workflows/erlang.yml/badge.svg)
![cpp workflow](https://github.com/helium/proto/actions/workflows/cpp.yml/badge.svg)

## Contributing

- Protobuf `message` definitions live in either:
  + `src/service/*.proto` if only used in gRPC service definitions, or
  + `src/*.proto` if used by service-free code or shared across gRPC services
- Avoid `float` in Protobufs because representations differ across hardware architectures
  + There are many floating point representations from IEEE, plus others
  + i.e., some range of interior digits are random per float spec
- Frequency should use `uint32` and should be in Hz
- rssi or signal is always negative, thus use `sint32` and is in deci-dbm (aka `ddbm`) which is `dbm * 10`
- snr is signal-to-noise ratio and should be `uint32`
- Fetch and share time in nanos, then truncate to appropriate granularity as needed
  + e.g., get from OS in nanos
- Reject any PR unless units are documented inline within Protobuf definition
- Document units of fields
- When exceptions to the above occur, please explain *why* within comments
