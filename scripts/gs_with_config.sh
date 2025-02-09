function datapoint() {
    # pub fn as_bytes(&self) -> [u8; 20] {
    #     let mut bytes = [0; 20];
    #     bytes[0] = 0xFF;
    #     bytes[1..3].copy_from_slice(&self.datatype.to_id().to_be_bytes());
    #     bytes[3..11].copy_from_slice(&self.value.to_le_bytes());
    #     bytes[11..19].copy_from_slice(&self.timestamp.to_le_bytes());
    #     bytes[19] = 0xFF;
    #     bytes
    # }

    printf "FF %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x 00 00 00 00 00 00 00 00 FF" $(($1 >> 8)) $(($1 & 0xFF)) $(($2 & 0xFF)) $((($2 >> 8) & 0xFF)) $((($2 >> 16) & 0xFF)) $((($2 >> 24) & 0xFF)) $((($2 >> 32) & 0xFF)) $((($2 >> 40) & 0xFF)) $((($2 >> 48) & 0xFF)) $((($2 >> 56) & 0xFF)) | xxd -r -p
}

# core.tx_channel.send(PodToGsMessage {
#             // 359
#             // 0xE981A1EA0B1A4199
#             dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
#         });
#         core.tx_channel.send(PodToGsMessage {
#             // 360
#             // 0xDEEDB95C8FC613FF
#             dp: Datapoint::new(Datatype::EventsHash, EVENTS_HASH, ticks()),
#         });
#         core.tx_channel.send(PodToGsMessage {
#             // 361
#             // 0xE1BC61029CE8A7B3
#             dp: Datapoint::new(Datatype::DataHash, DATA_HASH, ticks()),
#         });
#         core.tx_channel.send(PodToGsMessage {
#             // 369
#             // 0xB13F6E1D797FE777
#             dp: Datapoint::new(Datatype::ConfigHash, CONFIG_HASH, ticks()),
#         });
#         core.tx_channel.send(PodToGsMessage {
#             // 368
#             dp: Datapoint::new(Datatype::FrontendHeartbeating, 0, ticks()),
#         });

function handshakedatapoints() {
    # datapoint 359 0xE981A1EA0B1A4199
    # datapoint 360 0xDEEDB95C8FC613FF
    # datapoint 361 0xE1BC61029CE8A7B3
    # datapoint 369 0xB13F6E1D797FE777
    # datapoint 368 0

    datapoint 359 0xE981A1EA0B1A4192
    datapoint 360 0xDEEDB95C8FC613FF
    datapoint 361 0xE1BC61029CE8A7B3
    datapoint 369 0xB13F6E1D797FE777
    datapoint 368 0
}

# handshakedatapoints | nc 192.168.1.15 6949
handshakedatapoints | nc 127.0.0.1 6949
