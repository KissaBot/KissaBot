@test:
  cargo b --manifest-path=./test_plugin/Cargo.toml
  cargo b --manifest-path=./test_plugin_2/Cargo.toml
  cargo r