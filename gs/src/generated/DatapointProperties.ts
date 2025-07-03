
/* AUTO GENERATED USING npm run generate:gs */
export type NamedCommand = "SendHashes" | "LeviDropdown" | "DefaultCommand" | "GeneralEmergency" | "FSMUpdate" | "SystemCheck" | "ResetSenseCon" | "ResetPowertrain" | "ResetPropulsion" | "ResetLevitation" | "ResetLocalization" | "Heartbeat" | "FrontendHeartbeat" | "EmitEvent" | "StartHV" | "StopHV" | "LevitationOn" | "LevitationOff" | "PropulsionOn" | "PropulsionOff" | "SendPropulsionControlWord1" | "SendPropulsionControlWord2" | "PPControlParams" | "PPDebugParams1" | "PPDebugParams2" | "PPTestControlParams" | "PPRunParametersB" | "PPRunParameters1" | "PPRunParameters2" | "Shutdown" | "EmergencyBrake" | "SystemReset" | "RearmSDC" | "ConnectionEstablished" | "ConnectionClosed" | "MockPtAck" | "MockLeviAck" | "MockPropAck" | "MockHVOn" | "RequestFsmState";
export const NamedCommandValues:NamedCommand[] = [
"SendHashes", "LeviDropdown", "DefaultCommand", "GeneralEmergency", "FSMUpdate", "SystemCheck", "ResetSenseCon", "ResetPowertrain", "ResetPropulsion", "ResetLevitation", "ResetLocalization", "Heartbeat", "FrontendHeartbeat", "EmitEvent", "StartHV", "StopHV", "LevitationOn", "LevitationOff", "PropulsionOn", "PropulsionOff", "SendPropulsionControlWord1", "SendPropulsionControlWord2", "PPControlParams", "PPDebugParams1", "PPDebugParams2", "PPTestControlParams", "PPRunParametersB", "PPRunParameters1", "PPRunParameters2", "Shutdown", "EmergencyBrake", "SystemReset", "RearmSDC", "ConnectionEstablished", "ConnectionClosed", "MockPtAck", "MockLeviAck", "MockPropAck", "MockHVOn", "RequestFsmState"];

export type NamedDatatype = "TempMotorLeft0" | "TempMotorLeft1" | "TempMotorLeft2" | "TempMotorLeft3" | "TempMotorLeft4" | "TempMotorLeft5" | "TempMotorLeft6" | "TempMotorLeft7" | "TempMotorRight0" | "TempMotorRight1" | "TempMotorRight2" | "TempMotorRight3" | "TempMotorRight4" | "TempMotorRight5" | "TempMotorRight6" | "TempMotorRight7" | "PTCState" | "HVALState" | "IMDWarnings" | "Errors" | "BMSVoltageHigh" | "BMSVoltageLow" | "BMSTemperatureHigh" | "BMSTemperatureLow" | "VPack" | "IPack" | "VDCLink" | "LV_BMS_VoltageHigh" | "LV_BMS_VoltageLow" | "LV_BMS_TemperatureHigh" | "LV_BMS_TemperatureLow" | "LV_BMS_VPack" | "LV_BMS_VPackAlt" | "LV_BMS_IPack" | "LV_BMS_IPackAlt" | "InitCheck" | "LV_Batt_V1" | "LV_Batt_V2" | "LV_Batt_V3" | "LV_Batt_V4" | "LV_Batt_V5" | "LV_Batt_V6" | "LV_Batt_V7" | "LV_Batt_V8" | "LV_Batt_V9" | "LV_Batt_V10" | "LV_Batt_V11" | "LV_Batt_V12" | "LV_Batt_V13" | "LV_Batt_V14" | "LV_Batt_V15" | "LV_Batt_V16" | "LV_Batt_V17" | "LV_Batt_V18" | "LV_Batt_V19" | "LV_Batt_V20" | "LV_Batt_V21" | "LV_Batt_V22" | "LV_Batt_V23" | "LV_Batt_V24" | "LV_Batt_V25" | "LV_Batt_V26" | "LV_Batt_V27" | "LV_Batt_V28" | "LV_Batt_V29" | "LV_Batt_V30" | "LV_Batt_V31" | "LV_Batt_V32" | "LV_Batt_V33" | "LV_Batt_V34" | "LV_Batt_V35" | "LV_Batt_V36" | "LV_Batt_V37" | "LV_Batt_V38" | "LV_Batt_V39" | "LV_Batt_V40" | "LV_Batt_V41" | "LV_Batt_V42" | "LV_Batt_V43" | "LV_Batt_V44" | "LV_Batt_V45" | "LV_Batt_V46" | "LV_Batt_V47" | "LV_Batt_V48" | "LV_Batt_V49" | "LV_Batt_V50" | "LV_Batt_V51" | "LV_Batt_V52" | "LV_Batt_V53" | "LV_Batt_V54" | "LV_Batt_V55" | "LV_Batt_V56" | "LV_Batt_V57" | "LV_Batt_V58" | "LV_Batt_V59" | "LV_Batt_V60" | "LV_Batt_V61" | "LV_Batt_V62" | "LV_Batt_V63" | "LV_Batt_V64" | "LV_Batt_V65" | "LV_Batt_V66" | "LV_Batt_V67" | "LV_Batt_V68" | "LV_Batt_V69" | "LV_Batt_V70" | "LV_Batt_V71" | "LV_Batt_V72" | "LV_Batt_V73" | "LV_Batt_V74" | "LV_Batt_V75" | "LV_Batt_V76" | "LV_Batt_V77" | "LV_Batt_V78" | "LV_Batt_V79" | "LV_Batt_V80" | "LV_Batt_T1" | "LV_Batt_T2" | "LV_Batt_T3" | "LV_Batt_T4" | "LV_Batt_T5" | "LV_Batt_T6" | "LV_Batt_T7" | "LV_Batt_T8" | "LV_Batt_T9" | "LV_Batt_T10" | "LV_Batt_T11" | "LV_Batt_T12" | "LV_Batt_T13" | "LV_Batt_T14" | "LV_Batt_T15" | "LV_Batt_T16" | "LV_Batt_T17" | "LV_Batt_T18" | "LV_Batt_T19" | "LV_Batt_T20" | "LV_Batt_T21" | "LV_Batt_T22" | "LV_Batt_T23" | "LV_Batt_T24" | "LV_Batt_T25" | "LV_Batt_T26" | "LV_Batt_T27" | "LV_Batt_T28" | "LV_Batt_T29" | "LV_Batt_T30" | "LV_Batt_T31" | "LV_Batt_T32" | "LV_Batt_T33" | "LV_Batt_T34" | "LV_Batt_T35" | "LV_Batt_T36" | "LV_Batt_T37" | "LV_Batt_T38" | "LV_Batt_T39" | "LV_Batt_T40" | "LV_Batt_T41" | "LV_Batt_T42" | "LV_Batt_T43" | "LV_Batt_T44" | "LV_Batt_T45" | "LV_Batt_T46" | "LV_Batt_T47" | "LV_Batt_T48" | "LV_Batt_T49" | "LV_Batt_T50" | "LV_Batt_T51" | "LV_Batt_T52" | "LV_Batt_T53" | "LV_Batt_T54" | "LV_Batt_T55" | "LV_Batt_T56" | "LV_Batt_T57" | "LV_Batt_T58" | "LV_Batt_T59" | "LV_Batt_T60" | "LV_Batt_T61" | "LV_Batt_T62" | "LV_Batt_T63" | "LV_Batt_T64" | "LV_Batt_T65" | "LV_Batt_T66" | "LV_Batt_T67" | "LV_Batt_T68" | "LV_Batt_T69" | "LV_Batt_T70" | "LV_Batt_T71" | "LV_Batt_T72" | "LV_Batt_T73" | "LV_Batt_T74" | "LV_Batt_T75" | "LV_Batt_T76" | "LV_Batt_T77" | "LV_Batt_T78" | "LV_Batt_T79" | "LV_Batt_T80" | "TempRangeStart" | "TempRangeEnd" | "Localization" | "Velocity" | "PPInitFault1" | "PPInitFault2" | "PPEmergency1" | "PPEmergency2" | "PTCError" | "BMSError" | "Word1" | "Word2" | "IqMeasured1" | "IqReference1" | "IdMeasured1" | "IdReference1" | "IqMeasured2" | "IqReference2" | "IdMeasured2" | "IdReference2" | "Vq_Log1" | "Vd_Log1" | "Vbus1" | "Ibus1" | "CANLog" | "Vq_Log2" | "Vd_Log2" | "Vbus2" | "Ibus2" | "Ta1" | "Tb1" | "Tc1" | "TCASE1" | "Ta2" | "Tb2" | "Tc2" | "TCASE2" | "FSMAckProp1" | "FSMAckProp2" | "FSMAckLevi" | "ClearFaultAckLevi" | "Offset1" | "Offset2" | "Offset3" | "Offset4" | "LeviFault" | "LeviHeartbeat" | "LeviFSMStateChanged" | "LevitationState" | "NonCriticalLeviError" | "Vertical" | "Lateral" | "Roll" | "Pitch" | "Yaw" | "VFL1" | "VFL2" | "VFR1" | "VFR2" | "VBL1" | "VBL2" | "VBR1" | "VBR2" | "LF1" | "LF2" | "LB1" | "LB2" | "ZRequested" | "RollRequested" | "PitchRequested" | "YRequested" | "YawRequested" | "TempHEMS1" | "TempHEMS2" | "TempHEMS3" | "TempHEMS4" | "TempHEMS5" | "TempHEMS6" | "TempHEMS7" | "TempHEMS8" | "TempEMS1" | "TempEMS2" | "TempEMS3" | "TempEMS4" | "TempEMS5" | "TempEMS6" | "TempEMS7" | "TempEMS8" | "DefaultDatatype" | "CommandHash" | "DataHash" | "ConfigHash" | "ValueError" | "ValueWarning" | "ValueCausedBraking" | "LocalisationHeartbeat" | "SensorHubHeartbeat" | "FrontendHeartbeating" | "FSMState" | "FSMTransitionFail" | "Emergency";

export const NamedDatatypeValues = [
"TempMotorLeft0", "TempMotorLeft1", "TempMotorLeft2", "TempMotorLeft3", "TempMotorLeft4", "TempMotorLeft5", "TempMotorLeft6", "TempMotorLeft7", "TempMotorRight0", "TempMotorRight1", "TempMotorRight2", "TempMotorRight3", "TempMotorRight4", "TempMotorRight5", "TempMotorRight6", "TempMotorRight7", "PTCState", "HVALState", "IMDWarnings", "Errors", "BMSVoltageHigh", "BMSVoltageLow", "BMSTemperatureHigh", "BMSTemperatureLow", "VPack", "IPack", "VDCLink", "LV_BMS_VoltageHigh", "LV_BMS_VoltageLow", "LV_BMS_TemperatureHigh", "LV_BMS_TemperatureLow", "LV_BMS_VPack", "LV_BMS_VPackAlt", "LV_BMS_IPack", "LV_BMS_IPackAlt", "InitCheck", "LV_Batt_V1", "LV_Batt_V2", "LV_Batt_V3", "LV_Batt_V4", "LV_Batt_V5", "LV_Batt_V6", "LV_Batt_V7", "LV_Batt_V8", "LV_Batt_V9", "LV_Batt_V10", "LV_Batt_V11", "LV_Batt_V12", "LV_Batt_V13", "LV_Batt_V14", "LV_Batt_V15", "LV_Batt_V16", "LV_Batt_V17", "LV_Batt_V18", "LV_Batt_V19", "LV_Batt_V20", "LV_Batt_V21", "LV_Batt_V22", "LV_Batt_V23", "LV_Batt_V24", "LV_Batt_V25", "LV_Batt_V26", "LV_Batt_V27", "LV_Batt_V28", "LV_Batt_V29", "LV_Batt_V30", "LV_Batt_V31", "LV_Batt_V32", "LV_Batt_V33", "LV_Batt_V34", "LV_Batt_V35", "LV_Batt_V36", "LV_Batt_V37", "LV_Batt_V38", "LV_Batt_V39", "LV_Batt_V40", "LV_Batt_V41", "LV_Batt_V42", "LV_Batt_V43", "LV_Batt_V44", "LV_Batt_V45", "LV_Batt_V46", "LV_Batt_V47", "LV_Batt_V48", "LV_Batt_V49", "LV_Batt_V50", "LV_Batt_V51", "LV_Batt_V52", "LV_Batt_V53", "LV_Batt_V54", "LV_Batt_V55", "LV_Batt_V56", "LV_Batt_V57", "LV_Batt_V58", "LV_Batt_V59", "LV_Batt_V60", "LV_Batt_V61", "LV_Batt_V62", "LV_Batt_V63", "LV_Batt_V64", "LV_Batt_V65", "LV_Batt_V66", "LV_Batt_V67", "LV_Batt_V68", "LV_Batt_V69", "LV_Batt_V70", "LV_Batt_V71", "LV_Batt_V72", "LV_Batt_V73", "LV_Batt_V74", "LV_Batt_V75", "LV_Batt_V76", "LV_Batt_V77", "LV_Batt_V78", "LV_Batt_V79", "LV_Batt_V80", "LV_Batt_T1", "LV_Batt_T2", "LV_Batt_T3", "LV_Batt_T4", "LV_Batt_T5", "LV_Batt_T6", "LV_Batt_T7", "LV_Batt_T8", "LV_Batt_T9", "LV_Batt_T10", "LV_Batt_T11", "LV_Batt_T12", "LV_Batt_T13", "LV_Batt_T14", "LV_Batt_T15", "LV_Batt_T16", "LV_Batt_T17", "LV_Batt_T18", "LV_Batt_T19", "LV_Batt_T20", "LV_Batt_T21", "LV_Batt_T22", "LV_Batt_T23", "LV_Batt_T24", "LV_Batt_T25", "LV_Batt_T26", "LV_Batt_T27", "LV_Batt_T28", "LV_Batt_T29", "LV_Batt_T30", "LV_Batt_T31", "LV_Batt_T32", "LV_Batt_T33", "LV_Batt_T34", "LV_Batt_T35", "LV_Batt_T36", "LV_Batt_T37", "LV_Batt_T38", "LV_Batt_T39", "LV_Batt_T40", "LV_Batt_T41", "LV_Batt_T42", "LV_Batt_T43", "LV_Batt_T44", "LV_Batt_T45", "LV_Batt_T46", "LV_Batt_T47", "LV_Batt_T48", "LV_Batt_T49", "LV_Batt_T50", "LV_Batt_T51", "LV_Batt_T52", "LV_Batt_T53", "LV_Batt_T54", "LV_Batt_T55", "LV_Batt_T56", "LV_Batt_T57", "LV_Batt_T58", "LV_Batt_T59", "LV_Batt_T60", "LV_Batt_T61", "LV_Batt_T62", "LV_Batt_T63", "LV_Batt_T64", "LV_Batt_T65", "LV_Batt_T66", "LV_Batt_T67", "LV_Batt_T68", "LV_Batt_T69", "LV_Batt_T70", "LV_Batt_T71", "LV_Batt_T72", "LV_Batt_T73", "LV_Batt_T74", "LV_Batt_T75", "LV_Batt_T76", "LV_Batt_T77", "LV_Batt_T78", "LV_Batt_T79", "LV_Batt_T80", "TempRangeStart", "TempRangeEnd", "Localization", "Velocity", "PPInitFault1", "PPInitFault2", "PPEmergency1", "PPEmergency2", "PTCError", "BMSError", "Word1", "Word2", "IqMeasured1", "IqReference1", "IdMeasured1", "IdReference1", "IqMeasured2", "IqReference2", "IdMeasured2", "IdReference2", "Vq_Log1", "Vd_Log1", "Vbus1", "Ibus1", "CANLog", "Vq_Log2", "Vd_Log2", "Vbus2", "Ibus2", "Ta1", "Tb1", "Tc1", "TCASE1", "Ta2", "Tb2", "Tc2", "TCASE2", "FSMAckProp1", "FSMAckProp2", "FSMAckLevi", "ClearFaultAckLevi", "Offset1", "Offset2", "Offset3", "Offset4", "LeviFault", "LeviHeartbeat", "LeviFSMStateChanged", "LevitationState", "NonCriticalLeviError", "Vertical", "Lateral", "Roll", "Pitch", "Yaw", "VFL1", "VFL2", "VFR1", "VFR2", "VBL1", "VBL2", "VBR1", "VBR2", "LF1", "LF2", "LB1", "LB2", "ZRequested", "RollRequested", "PitchRequested", "YRequested", "YawRequested", "TempHEMS1", "TempHEMS2", "TempHEMS3", "TempHEMS4", "TempHEMS5", "TempHEMS6", "TempHEMS7", "TempHEMS8", "TempEMS1", "TempEMS2", "TempEMS3", "TempEMS4", "TempEMS5", "TempEMS6", "TempEMS7", "TempEMS8", "DefaultDatatype", "CommandHash", "DataHash", "ConfigHash", "ValueError", "ValueWarning", "ValueCausedBraking", "LocalisationHeartbeat", "SensorHubHeartbeat", "FrontendHeartbeating", "FSMState", "FSMTransitionFail", "Emergency"];


        // gdd stores registration
        // auto-generated with npm run generate:gs
        

            gdd.stores.registerStore<number>("TempMotorLeft0", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft3", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft4", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(5, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft5", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(6, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft6", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(7, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorLeft7", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Left")!.addEntry(8, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight0", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight3", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight4", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(5, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight5", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(6, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight6", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(7, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempMotorRight7", 0, data => {
    const curr = Number(data);
    $chartStore.get("Motor Temperatures Right")!.addEntry(8, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("PTCState", 0);

            gdd.stores.registerStore<number>("HVALState", 0);

            gdd.stores.registerStore<number>("IMDWarnings", 0);

            gdd.stores.registerStore<number>("Errors", 0);

            gdd.stores.registerStore<number>("BMSVoltageHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltage High")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("BMSVoltageLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltage Low")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("BMSTemperatureHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temps")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("BMSTemperatureLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temps")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VPack", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Logs")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IPack", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Logs")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VDCLink", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Logs")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LV_BMS_VoltageHigh", 0);

            gdd.stores.registerStore<number>("LV_BMS_VoltageLow", 0);

            gdd.stores.registerStore<number>("LV_BMS_TemperatureHigh", 0);

            gdd.stores.registerStore<number>("LV_BMS_TemperatureLow", 0);

            gdd.stores.registerStore<number>("LV_BMS_VPack", 0);

            gdd.stores.registerStore<number>("LV_BMS_VPackAlt", 0);

            gdd.stores.registerStore<number>("LV_BMS_IPack", 0);

            gdd.stores.registerStore<number>("LV_BMS_IPackAlt", 0);

            gdd.stores.registerStore<number>("InitCheck", 0);

            gdd.stores.registerStore<number>("LV_Batt_V1", 0);

            gdd.stores.registerStore<number>("LV_Batt_V2", 0);

            gdd.stores.registerStore<number>("LV_Batt_V3", 0);

            gdd.stores.registerStore<number>("LV_Batt_V4", 0);

            gdd.stores.registerStore<number>("LV_Batt_V5", 0);

            gdd.stores.registerStore<number>("LV_Batt_V6", 0);

            gdd.stores.registerStore<number>("LV_Batt_V7", 0);

            gdd.stores.registerStore<number>("LV_Batt_V8", 0);

            gdd.stores.registerStore<number>("LV_Batt_V9", 0);

            gdd.stores.registerStore<number>("LV_Batt_V10", 0);

            gdd.stores.registerStore<number>("LV_Batt_V11", 0);

            gdd.stores.registerStore<number>("LV_Batt_V12", 0);

            gdd.stores.registerStore<number>("LV_Batt_V13", 0);

            gdd.stores.registerStore<number>("LV_Batt_V14", 0);

            gdd.stores.registerStore<number>("LV_Batt_V15", 0);

            gdd.stores.registerStore<number>("LV_Batt_V16", 0);

            gdd.stores.registerStore<number>("LV_Batt_V17", 0);

            gdd.stores.registerStore<number>("LV_Batt_V18", 0);

            gdd.stores.registerStore<number>("LV_Batt_V19", 0);

            gdd.stores.registerStore<number>("LV_Batt_V20", 0);

            gdd.stores.registerStore<number>("LV_Batt_V21", 0);

            gdd.stores.registerStore<number>("LV_Batt_V22", 0);

            gdd.stores.registerStore<number>("LV_Batt_V23", 0);

            gdd.stores.registerStore<number>("LV_Batt_V24", 0);

            gdd.stores.registerStore<number>("LV_Batt_V25", 0);

            gdd.stores.registerStore<number>("LV_Batt_V26", 0);

            gdd.stores.registerStore<number>("LV_Batt_V27", 0);

            gdd.stores.registerStore<number>("LV_Batt_V28", 0);

            gdd.stores.registerStore<number>("LV_Batt_V29", 0);

            gdd.stores.registerStore<number>("LV_Batt_V30", 0);

            gdd.stores.registerStore<number>("LV_Batt_V31", 0);

            gdd.stores.registerStore<number>("LV_Batt_V32", 0);

            gdd.stores.registerStore<number>("LV_Batt_V33", 0);

            gdd.stores.registerStore<number>("LV_Batt_V34", 0);

            gdd.stores.registerStore<number>("LV_Batt_V35", 0);

            gdd.stores.registerStore<number>("LV_Batt_V36", 0);

            gdd.stores.registerStore<number>("LV_Batt_V37", 0);

            gdd.stores.registerStore<number>("LV_Batt_V38", 0);

            gdd.stores.registerStore<number>("LV_Batt_V39", 0);

            gdd.stores.registerStore<number>("LV_Batt_V40", 0);

            gdd.stores.registerStore<number>("LV_Batt_V41", 0);

            gdd.stores.registerStore<number>("LV_Batt_V42", 0);

            gdd.stores.registerStore<number>("LV_Batt_V43", 0);

            gdd.stores.registerStore<number>("LV_Batt_V44", 0);

            gdd.stores.registerStore<number>("LV_Batt_V45", 0);

            gdd.stores.registerStore<number>("LV_Batt_V46", 0);

            gdd.stores.registerStore<number>("LV_Batt_V47", 0);

            gdd.stores.registerStore<number>("LV_Batt_V48", 0);

            gdd.stores.registerStore<number>("LV_Batt_V49", 0);

            gdd.stores.registerStore<number>("LV_Batt_V50", 0);

            gdd.stores.registerStore<number>("LV_Batt_V51", 0);

            gdd.stores.registerStore<number>("LV_Batt_V52", 0);

            gdd.stores.registerStore<number>("LV_Batt_V53", 0);

            gdd.stores.registerStore<number>("LV_Batt_V54", 0);

            gdd.stores.registerStore<number>("LV_Batt_V55", 0);

            gdd.stores.registerStore<number>("LV_Batt_V56", 0);

            gdd.stores.registerStore<number>("LV_Batt_V57", 0);

            gdd.stores.registerStore<number>("LV_Batt_V58", 0);

            gdd.stores.registerStore<number>("LV_Batt_V59", 0);

            gdd.stores.registerStore<number>("LV_Batt_V60", 0);

            gdd.stores.registerStore<number>("LV_Batt_V61", 0);

            gdd.stores.registerStore<number>("LV_Batt_V62", 0);

            gdd.stores.registerStore<number>("LV_Batt_V63", 0);

            gdd.stores.registerStore<number>("LV_Batt_V64", 0);

            gdd.stores.registerStore<number>("LV_Batt_V65", 0);

            gdd.stores.registerStore<number>("LV_Batt_V66", 0);

            gdd.stores.registerStore<number>("LV_Batt_V67", 0);

            gdd.stores.registerStore<number>("LV_Batt_V68", 0);

            gdd.stores.registerStore<number>("LV_Batt_V69", 0);

            gdd.stores.registerStore<number>("LV_Batt_V70", 0);

            gdd.stores.registerStore<number>("LV_Batt_V71", 0);

            gdd.stores.registerStore<number>("LV_Batt_V72", 0);

            gdd.stores.registerStore<number>("LV_Batt_V73", 0);

            gdd.stores.registerStore<number>("LV_Batt_V74", 0);

            gdd.stores.registerStore<number>("LV_Batt_V75", 0);

            gdd.stores.registerStore<number>("LV_Batt_V76", 0);

            gdd.stores.registerStore<number>("LV_Batt_V77", 0);

            gdd.stores.registerStore<number>("LV_Batt_V78", 0);

            gdd.stores.registerStore<number>("LV_Batt_V79", 0);

            gdd.stores.registerStore<number>("LV_Batt_V80", 0);

            gdd.stores.registerStore<number>("LV_Batt_T1", 0);

            gdd.stores.registerStore<number>("LV_Batt_T2", 0);

            gdd.stores.registerStore<number>("LV_Batt_T3", 0);

            gdd.stores.registerStore<number>("LV_Batt_T4", 0);

            gdd.stores.registerStore<number>("LV_Batt_T5", 0);

            gdd.stores.registerStore<number>("LV_Batt_T6", 0);

            gdd.stores.registerStore<number>("LV_Batt_T7", 0);

            gdd.stores.registerStore<number>("LV_Batt_T8", 0);

            gdd.stores.registerStore<number>("LV_Batt_T9", 0);

            gdd.stores.registerStore<number>("LV_Batt_T10", 0);

            gdd.stores.registerStore<number>("LV_Batt_T11", 0);

            gdd.stores.registerStore<number>("LV_Batt_T12", 0);

            gdd.stores.registerStore<number>("LV_Batt_T13", 0);

            gdd.stores.registerStore<number>("LV_Batt_T14", 0);

            gdd.stores.registerStore<number>("LV_Batt_T15", 0);

            gdd.stores.registerStore<number>("LV_Batt_T16", 0);

            gdd.stores.registerStore<number>("LV_Batt_T17", 0);

            gdd.stores.registerStore<number>("LV_Batt_T18", 0);

            gdd.stores.registerStore<number>("LV_Batt_T19", 0);

            gdd.stores.registerStore<number>("LV_Batt_T20", 0);

            gdd.stores.registerStore<number>("LV_Batt_T21", 0);

            gdd.stores.registerStore<number>("LV_Batt_T22", 0);

            gdd.stores.registerStore<number>("LV_Batt_T23", 0);

            gdd.stores.registerStore<number>("LV_Batt_T24", 0);

            gdd.stores.registerStore<number>("LV_Batt_T25", 0);

            gdd.stores.registerStore<number>("LV_Batt_T26", 0);

            gdd.stores.registerStore<number>("LV_Batt_T27", 0);

            gdd.stores.registerStore<number>("LV_Batt_T28", 0);

            gdd.stores.registerStore<number>("LV_Batt_T29", 0);

            gdd.stores.registerStore<number>("LV_Batt_T30", 0);

            gdd.stores.registerStore<number>("LV_Batt_T31", 0);

            gdd.stores.registerStore<number>("LV_Batt_T32", 0);

            gdd.stores.registerStore<number>("LV_Batt_T33", 0);

            gdd.stores.registerStore<number>("LV_Batt_T34", 0);

            gdd.stores.registerStore<number>("LV_Batt_T35", 0);

            gdd.stores.registerStore<number>("LV_Batt_T36", 0);

            gdd.stores.registerStore<number>("LV_Batt_T37", 0);

            gdd.stores.registerStore<number>("LV_Batt_T38", 0);

            gdd.stores.registerStore<number>("LV_Batt_T39", 0);

            gdd.stores.registerStore<number>("LV_Batt_T40", 0);

            gdd.stores.registerStore<number>("LV_Batt_T41", 0);

            gdd.stores.registerStore<number>("LV_Batt_T42", 0);

            gdd.stores.registerStore<number>("LV_Batt_T43", 0);

            gdd.stores.registerStore<number>("LV_Batt_T44", 0);

            gdd.stores.registerStore<number>("LV_Batt_T45", 0);

            gdd.stores.registerStore<number>("LV_Batt_T46", 0);

            gdd.stores.registerStore<number>("LV_Batt_T47", 0);

            gdd.stores.registerStore<number>("LV_Batt_T48", 0);

            gdd.stores.registerStore<number>("LV_Batt_T49", 0);

            gdd.stores.registerStore<number>("LV_Batt_T50", 0);

            gdd.stores.registerStore<number>("LV_Batt_T51", 0);

            gdd.stores.registerStore<number>("LV_Batt_T52", 0);

            gdd.stores.registerStore<number>("LV_Batt_T53", 0);

            gdd.stores.registerStore<number>("LV_Batt_T54", 0);

            gdd.stores.registerStore<number>("LV_Batt_T55", 0);

            gdd.stores.registerStore<number>("LV_Batt_T56", 0);

            gdd.stores.registerStore<number>("LV_Batt_T57", 0);

            gdd.stores.registerStore<number>("LV_Batt_T58", 0);

            gdd.stores.registerStore<number>("LV_Batt_T59", 0);

            gdd.stores.registerStore<number>("LV_Batt_T60", 0);

            gdd.stores.registerStore<number>("LV_Batt_T61", 0);

            gdd.stores.registerStore<number>("LV_Batt_T62", 0);

            gdd.stores.registerStore<number>("LV_Batt_T63", 0);

            gdd.stores.registerStore<number>("LV_Batt_T64", 0);

            gdd.stores.registerStore<number>("LV_Batt_T65", 0);

            gdd.stores.registerStore<number>("LV_Batt_T66", 0);

            gdd.stores.registerStore<number>("LV_Batt_T67", 0);

            gdd.stores.registerStore<number>("LV_Batt_T68", 0);

            gdd.stores.registerStore<number>("LV_Batt_T69", 0);

            gdd.stores.registerStore<number>("LV_Batt_T70", 0);

            gdd.stores.registerStore<number>("LV_Batt_T71", 0);

            gdd.stores.registerStore<number>("LV_Batt_T72", 0);

            gdd.stores.registerStore<number>("LV_Batt_T73", 0);

            gdd.stores.registerStore<number>("LV_Batt_T74", 0);

            gdd.stores.registerStore<number>("LV_Batt_T75", 0);

            gdd.stores.registerStore<number>("LV_Batt_T76", 0);

            gdd.stores.registerStore<number>("LV_Batt_T77", 0);

            gdd.stores.registerStore<number>("LV_Batt_T78", 0);

            gdd.stores.registerStore<number>("LV_Batt_T79", 0);

            gdd.stores.registerStore<number>("LV_Batt_T80", 0);

            gdd.stores.registerStore<number>("TempRangeStart", 0);

            gdd.stores.registerStore<number>("TempRangeEnd", 0);

            gdd.stores.registerStore<number>("Localization", 0, data => {
    const curr = Number(data);
    $chartStore.get("Localization")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Velocity", 0, data => {
    const curr = Number(data);
    $chartStore.get("Velocity")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("PPInitFault1", 255);

            gdd.stores.registerStore<number>("PPInitFault2", 255);

            gdd.stores.registerStore<number>("PPEmergency1", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Logs")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("PPEmergency2", 0);

            gdd.stores.registerStore<number>("PTCError", 0);

            gdd.stores.registerStore<number>("BMSError", 0);

            gdd.stores.registerStore<number>("Word1", 0);

            gdd.stores.registerStore<number>("Word2", 0);

            gdd.stores.registerStore<number>("IqMeasured1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IqReference1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IdMeasured1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IdReference1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IqMeasured2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IqReference2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IdMeasured2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("IdReference2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Vq_Log1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Vd_Log1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Vbus1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Ibus1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("CANLog", 0);

            gdd.stores.registerStore<number>("Vq_Log2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Vd_Log2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Vbus2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Ibus2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Ta1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Tb1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Tc1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TCASE1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Ta2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Tb2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Tc2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TCASE2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("FSMAckProp1", 0);

            gdd.stores.registerStore<number>("FSMAckProp2", 0);

            gdd.stores.registerStore<number>("FSMAckLevi", 0);

            gdd.stores.registerStore<number>("ClearFaultAckLevi", 0);

            gdd.stores.registerStore<number>("Offset1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Offset")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Offset2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Offset")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Offset3", 0, data => {
    const curr = Number(data);
    $chartStore.get("Offset")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Offset4", 0, data => {
    const curr = Number(data);
    $chartStore.get("Offset")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LeviFault", 0);

            gdd.stores.registerStore<number>("LeviHeartbeat", 0);

            gdd.stores.registerStore<number>("LeviFSMStateChanged", 0);

            gdd.stores.registerStore<number>("LevitationState", 0);

            gdd.stores.registerStore<number>("NonCriticalLeviError", 0);

            gdd.stores.registerStore<number>("Vertical", 0, data => {
    const curr = Number(data);
    $chartStore.get("Air Gaps")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Lateral", 0, data => {
    const curr = Number(data);
    $chartStore.get("Air Gaps")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Roll", 0, data => {
    const curr = Number(data);
    $chartStore.get("Angles")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Pitch", 0, data => {
    const curr = Number(data);
    $chartStore.get("Angles")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("Yaw", 0, data => {
    const curr = Number(data);
    $chartStore.get("Angles")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VFL1", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VFL2", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VFR1", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VFR2", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VBL1", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(5, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VBL2", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(6, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VBR1", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(7, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("VBR2", 0, data => {
    const curr = Number(data);
    $chartStore.get("HEMS Current")!.addEntry(8, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LF1", 0, data => {
    const curr = Number(data);
    $chartStore.get("EMS Current")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LF2", 0, data => {
    const curr = Number(data);
    $chartStore.get("EMS Current")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LB1", 0, data => {
    const curr = Number(data);
    $chartStore.get("EMS Current")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("LB2", 0, data => {
    const curr = Number(data);
    $chartStore.get("EMS Current")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("ZRequested", 0, data => {
    const curr = Number(data);
    $chartStore.get("Requested Force Vertical")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("RollRequested", 0, data => {
    const curr = Number(data);
    $chartStore.get("Requested Force Vertical")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("PitchRequested", 0, data => {
    const curr = Number(data);
    $chartStore.get("Requested Force Vertical")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("YRequested", 0, data => {
    const curr = Number(data);
    $chartStore.get("Requested Force Horizontal")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("YawRequested", 0, data => {
    const curr = Number(data);
    $chartStore.get("Requested Force Horizontal")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS3", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS4", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS5", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(5, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS6", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(6, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS7", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(7, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempHEMS8", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures HEMS")!.addEntry(8, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS1", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(1, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS2", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(2, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS3", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(3, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS4", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(4, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS5", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(5, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS6", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(6, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS7", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(7, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("TempEMS8", 0, data => {
    const curr = Number(data);
    $chartStore.get("Temperatures EMS")!.addEntry(8, curr);
    return curr;
}
);

            gdd.stores.registerStore<number>("FSMState", 0);

            gdd.stores.registerStore<number>("FSMTransitionFail", 100);

            gdd.stores.registerStore<number>("Emergency", 0);

export const DatapointProperties: Record<NamedDatatype, {
    lower: number | null,
    upper: number | null,
    stale_after: number | null,
    critical: boolean | null
}> = {
"TempMotorLeft0": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft1": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft2": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft3": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft4": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft5": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft6": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorLeft7": { lower: null, upper: null, stale_after: null, critical: null },
"TempMotorRight0": { lower: null, upper: null, stale_after: null, critical: true },
"TempMotorRight1": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight2": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight3": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight4": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight5": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight6": { lower: null, upper: null, stale_after: null, critical: false },
"TempMotorRight7": { lower: null, upper: null, stale_after: null, critical: false },
"PTCState": { lower: null, upper: null, stale_after: null, critical: null },
"HVALState": { lower: null, upper: null, stale_after: null, critical: null },
"IMDWarnings": { lower: null, upper: null, stale_after: null, critical: true },
"Errors": { lower: null, upper: null, stale_after: null, critical: true },
"BMSVoltageHigh": { lower: null, upper: null, stale_after: null, critical: true },
"BMSVoltageLow": { lower: null, upper: null, stale_after: null, critical: true },
"BMSTemperatureHigh": { lower: null, upper: null, stale_after: null, critical: true },
"BMSTemperatureLow": { lower: null, upper: null, stale_after: null, critical: true },
"VPack": { lower: null, upper: null, stale_after: null, critical: null },
"IPack": { lower: null, upper: null, stale_after: null, critical: true },
"VDCLink": { lower: null, upper: null, stale_after: null, critical: null },
"LV_BMS_VoltageHigh": { lower: null, upper: null, stale_after: null, critical: true },
"LV_BMS_VoltageLow": { lower: null, upper: null, stale_after: null, critical: true },
"LV_BMS_TemperatureHigh": { lower: null, upper: null, stale_after: null, critical: true },
"LV_BMS_TemperatureLow": { lower: null, upper: null, stale_after: null, critical: true },
"LV_BMS_VPack": { lower: null, upper: null, stale_after: null, critical: null },
"LV_BMS_VPackAlt": { lower: null, upper: null, stale_after: null, critical: null },
"LV_BMS_IPack": { lower: null, upper: null, stale_after: null, critical: null },
"LV_BMS_IPackAlt": { lower: null, upper: null, stale_after: null, critical: null },
"InitCheck": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V1": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V2": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V3": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V4": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V5": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V6": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V7": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V8": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V9": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V10": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V11": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V12": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V13": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V14": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V15": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V16": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V17": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V18": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V19": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V20": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V21": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V22": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V23": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V24": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V25": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V26": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V27": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V28": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V29": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V30": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V31": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V32": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V33": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V34": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V35": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V36": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V37": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V38": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V39": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V40": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V41": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V42": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V43": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V44": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V45": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V46": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V47": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V48": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V49": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V50": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V51": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V52": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V53": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V54": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V55": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V56": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V57": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V58": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V59": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V60": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V61": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V62": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V63": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V64": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V65": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V66": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V67": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V68": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V69": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V70": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V71": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V72": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V73": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V74": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V75": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V76": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V77": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V78": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V79": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_V80": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T1": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T2": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T3": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T4": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T5": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T6": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T7": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T8": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T9": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T10": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T11": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T12": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T13": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T14": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T15": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T16": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T17": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T18": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T19": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T20": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T21": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T22": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T23": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T24": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T25": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T26": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T27": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T28": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T29": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T30": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T31": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T32": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T33": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T34": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T35": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T36": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T37": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T38": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T39": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T40": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T41": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T42": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T43": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T44": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T45": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T46": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T47": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T48": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T49": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T50": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T51": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T52": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T53": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T54": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T55": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T56": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T57": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T58": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T59": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T60": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T61": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T62": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T63": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T64": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T65": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T66": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T67": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T68": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T69": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T70": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T71": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T72": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T73": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T74": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T75": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T76": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T77": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T78": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T79": { lower: null, upper: null, stale_after: null, critical: null },
"LV_Batt_T80": { lower: null, upper: null, stale_after: null, critical: null },
"TempRangeStart": { lower: null, upper: null, stale_after: null, critical: null },
"TempRangeEnd": { lower: null, upper: null, stale_after: null, critical: null },
"Localization": { lower: null, upper: null, stale_after: null, critical: null },
"Velocity": { lower: null, upper: null, stale_after: null, critical: null },
"PPInitFault1": { lower: null, upper: null, stale_after: null, critical: null },
"PPInitFault2": { lower: null, upper: null, stale_after: null, critical: null },
"PPEmergency1": { lower: null, upper: null, stale_after: null, critical: true },
"PPEmergency2": { lower: null, upper: null, stale_after: null, critical: true },
"PTCError": { lower: null, upper: null, stale_after: null, critical: true },
"BMSError": { lower: null, upper: null, stale_after: null, critical: true },
"Word1": { lower: null, upper: null, stale_after: null, critical: null },
"Word2": { lower: null, upper: null, stale_after: null, critical: null },
"IqMeasured1": { lower: null, upper: null, stale_after: null, critical: null },
"IqReference1": { lower: null, upper: null, stale_after: null, critical: null },
"IdMeasured1": { lower: null, upper: null, stale_after: null, critical: null },
"IdReference1": { lower: null, upper: null, stale_after: null, critical: null },
"IqMeasured2": { lower: null, upper: null, stale_after: null, critical: null },
"IqReference2": { lower: null, upper: null, stale_after: null, critical: null },
"IdMeasured2": { lower: null, upper: null, stale_after: null, critical: null },
"IdReference2": { lower: null, upper: null, stale_after: null, critical: null },
"Vq_Log1": { lower: null, upper: null, stale_after: null, critical: null },
"Vd_Log1": { lower: null, upper: null, stale_after: null, critical: null },
"Vbus1": { lower: null, upper: null, stale_after: null, critical: null },
"Ibus1": { lower: null, upper: null, stale_after: null, critical: null },
"CANLog": { lower: null, upper: null, stale_after: null, critical: null },
"Vq_Log2": { lower: null, upper: null, stale_after: null, critical: null },
"Vd_Log2": { lower: null, upper: null, stale_after: null, critical: null },
"Vbus2": { lower: null, upper: null, stale_after: null, critical: null },
"Ibus2": { lower: null, upper: null, stale_after: null, critical: null },
"Ta1": { lower: null, upper: null, stale_after: null, critical: null },
"Tb1": { lower: null, upper: null, stale_after: null, critical: null },
"Tc1": { lower: null, upper: null, stale_after: null, critical: null },
"TCASE1": { lower: null, upper: null, stale_after: null, critical: null },
"Ta2": { lower: null, upper: null, stale_after: null, critical: null },
"Tb2": { lower: null, upper: null, stale_after: null, critical: null },
"Tc2": { lower: null, upper: null, stale_after: null, critical: null },
"TCASE2": { lower: null, upper: null, stale_after: null, critical: null },
"FSMAckProp1": { lower: null, upper: null, stale_after: null, critical: null },
"FSMAckProp2": { lower: null, upper: null, stale_after: null, critical: null },
"FSMAckLevi": { lower: null, upper: null, stale_after: null, critical: null },
"ClearFaultAckLevi": { lower: null, upper: null, stale_after: null, critical: null },
"Offset1": { lower: null, upper: null, stale_after: null, critical: null },
"Offset2": { lower: null, upper: null, stale_after: null, critical: null },
"Offset3": { lower: null, upper: null, stale_after: null, critical: null },
"Offset4": { lower: null, upper: null, stale_after: null, critical: null },
"LeviFault": { lower: null, upper: null, stale_after: null, critical: null },
"LeviHeartbeat": { lower: null, upper: null, stale_after: null, critical: null },
"LeviFSMStateChanged": { lower: null, upper: null, stale_after: null, critical: null },
"LevitationState": { lower: null, upper: null, stale_after: null, critical: null },
"NonCriticalLeviError": { lower: null, upper: null, stale_after: null, critical: null },
"Vertical": { lower: null, upper: null, stale_after: null, critical: null },
"Lateral": { lower: null, upper: null, stale_after: null, critical: null },
"Roll": { lower: null, upper: null, stale_after: null, critical: null },
"Pitch": { lower: null, upper: null, stale_after: null, critical: null },
"Yaw": { lower: null, upper: null, stale_after: null, critical: null },
"VFL1": { lower: null, upper: null, stale_after: null, critical: null },
"VFL2": { lower: null, upper: null, stale_after: null, critical: null },
"VFR1": { lower: null, upper: null, stale_after: null, critical: null },
"VFR2": { lower: null, upper: null, stale_after: null, critical: null },
"VBL1": { lower: null, upper: null, stale_after: null, critical: null },
"VBL2": { lower: null, upper: null, stale_after: null, critical: null },
"VBR1": { lower: null, upper: null, stale_after: null, critical: null },
"VBR2": { lower: null, upper: null, stale_after: null, critical: null },
"LF1": { lower: null, upper: null, stale_after: null, critical: null },
"LF2": { lower: null, upper: null, stale_after: null, critical: null },
"LB1": { lower: null, upper: null, stale_after: null, critical: null },
"LB2": { lower: null, upper: null, stale_after: null, critical: null },
"ZRequested": { lower: null, upper: null, stale_after: null, critical: null },
"RollRequested": { lower: null, upper: null, stale_after: null, critical: null },
"PitchRequested": { lower: null, upper: null, stale_after: null, critical: null },
"YRequested": { lower: null, upper: null, stale_after: null, critical: null },
"YawRequested": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS1": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS2": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS3": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS4": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS5": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS6": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS7": { lower: null, upper: null, stale_after: null, critical: null },
"TempHEMS8": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS1": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS2": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS3": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS4": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS5": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS6": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS7": { lower: null, upper: null, stale_after: null, critical: null },
"TempEMS8": { lower: null, upper: null, stale_after: null, critical: null },
"DefaultDatatype": { lower: null, upper: null, stale_after: null, critical: null },
"CommandHash": { lower: null, upper: null, stale_after: null, critical: null },
"DataHash": { lower: null, upper: null, stale_after: null, critical: null },
"ConfigHash": { lower: null, upper: null, stale_after: null, critical: null },
"ValueError": { lower: null, upper: null, stale_after: null, critical: null },
"ValueWarning": { lower: null, upper: null, stale_after: null, critical: null },
"ValueCausedBraking": { lower: null, upper: null, stale_after: null, critical: null },
"LocalisationHeartbeat": { lower: null, upper: null, stale_after: null, critical: null },
"SensorHubHeartbeat": { lower: null, upper: null, stale_after: null, critical: null },
"FrontendHeartbeating": { lower: null, upper: null, stale_after: null, critical: null },
"FSMState": { lower: null, upper: null, stale_after: null, critical: false },
"FSMTransitionFail": { lower: null, upper: null, stale_after: null, critical: null },
"Emergency": { lower: null, upper: null, stale_after: null, critical: null }
};


