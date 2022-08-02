# rust-nvidia-monitor

## Getting Started

### Dependencies
* nvidia-smi

### Installing
* cargo build --release
* cp target/release/nvmon /usr/bin/nvmon

### Usage
* ./nvmon
* nvmon
```
NVIDIA RTX 3070 Laptop
44,°C 17.70W 360MHz 461MB/8192MB

	utilization: 36%
100	 -                                                           -
90	 -                        ┌┐                                 -
80	 -              ┌┐        ││                                 -
70	 -              ││        ││                                 -
60	 -              ││        ││                                 -
50	 -              ││        ││           ┌──────────────┐      -
40	 -              ││        ││           │              │      -
30	 -              ││        ││ ┌──┐    ┌┐│              │    ┌─-
20	 -              ││       ┌┘└─┘  │    │││              │    │ -
10	 -              ││       │      │   ┌┘││              └┐  ┌┘ -
0	 -──────────────┘└───────┘      └───┘ └┘               └──┘  -
```
