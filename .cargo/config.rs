[target.thumbv7em-none-eabihf]
runner = 'gdb-multiarch'
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv6m-none-eabi"