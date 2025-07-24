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

---
config:
  theme: redux-color
---
mindmap
    root((DH09-sw/))
        ground station
            backend
                tcp connection
                    gs->pod
                    pod->gs
                frontend commands
                data processing
            frontend
                serpenta ui
            tui
                store data
                display
        pod
            fsm
            lib
                datapoint definitions
                fsm event definitions
                pod codegen
                    build.rs
                    lib.rs
            main
                panic handler
                matching_methods.rs
                    interpreting data
                    interpreting events
                main.rs
                    configure peripherals
                    spawn tasks
                task definitions
                    comms_tasks.rs
                can
                ethernet
                    logic.rs
                        tcp fsm
                        gs discovery
                        reconnecting
                    icmp ping
                        testing.rs
        config
            config files
                datatype defs and conversions
                    dataflow.yaml
                general configuration
                    config.toml
            code generation
                dataflow
                    functions for value conversions
                    datatypes
                    typescript code gen
                    levi code gen
                logs.rs
                    natural logarithm approximations
                limits.rs
                    upper and lower bounds for sensor values
```

FAQ:

- where does the name Theia come from?
    - see [https://en.wiktionary.org/wiki/θεία](https://en.wiktionary.org/wiki/θεία)
