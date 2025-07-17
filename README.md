# Delft Hyperloop: Theia (2024 – 2025)

https://www.delfthyperloop.nl/our-pods/theia

Welcome to the brains of Delft Hyperloop 09's pod: _Theia_.
This year, the team focused on joining the move towards hyperloop standardisation,
by scaling up the pod to comply with the track design of the 
[European Hyperloop Centre (EHC)](https://www.hyperloopcenter.eu).

For the second year in a row our code base is (proudly) open-source,
with the hope of inspiring other student teams when developing their hyperloop prototypes,
and to encourage discussion on how these control systems should be built.

# architecture

```mermaid
treemap-beta
"Ground Station"
    "Backend"
        "TCP Connection"
            "tcp: gs->pod task": 10
            "tcp: pod->gs task": 10
        "frontend commands": 10
        "data processing": 10
    "frontend"
        "serpenta ui (svelte)": 20
    "TUI"
        "store data": 5
        "display": 5
"Pod"
    "FSM": 20
    "lib"
        "datapoint definitions": 3
        "fsm event definitions": 3
        "pod-specific code gen"
            "build.rs": 10
            "lib.rs": 2
    "main"
        "panic handler": 10
        "matching_methods.rs"
            "interpreting data": 5
            "interpreting events": 5
        "main.rs"
            "configure peripherals": 5
            "spawn tasks": 5
        "task definitions"
            "comms_tasks.rs": 10
        "can"
        "ethernet"
            "logic.rs"
                "connection fsm": 4
                "gs discovery": 4
                "reconnecting": 4
            "icmp ping"
                "testing.rs": 2
"Config"
    "config files"
        "datatype defs and conversions"
            "dataflow.yaml": 10
        "general configuration"
            "config.toml": 5
    "code generation"
        "dataflow"
            "functions for value conversions": 3
            "datatypes": 3
            "typescript code gen": 2
            "levi code gen": 2
        "logs.rs"
            "natural logarithm (ln) approximations": 2
        "limits.rs"
            "upper and lower bounds for sensor values": 2
```

FAQ:

- where does the name Theia come from?
    - see [https://en.wiktionary.org/wiki/θεία](https://en.wiktionary.org/wiki/θεία)
