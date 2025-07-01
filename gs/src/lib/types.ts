import type { ModalSettings } from '@skeletonlabs/skeleton';

/* AUTO GENERATED USING npm run generate:gs */

export type NamedCommand = "SendHashes" | "LeviDropdown" | "DefaultCommand" | "GeneralEmergency" | "FSMUpdate" | "SystemCheck" | "ResetSenseCon" | "ResetPowertrain" | "ResetPropulsion" | "ResetLevitation" | "ResetLocalization" | "Heartbeat" | "FrontendHeartbeat" | "EmitEvent" | "StartHV" | "StopHV" | "LevitationOn" | "LevitationOff" | "PropulsionOn" | "PropulsionOff" | "SendPropulsionControlWord1" | "SendPropulsionControlWord2" | "PPControlParams" | "PPDebugParams1" | "PPDebugParams2" | "PPTestControlParams" | "PPRunParametersB" | "PPRunParameters1" | "PPRunParameters2" | "Shutdown" | "EmergencyBrake" | "SystemReset" | "RearmSDC" | "ConnectionEstablished" | "ConnectionClosed" | "MockPtAck" | "MockLeviAck" | "MockPropAck" | "MockHVOn" | "RequestFsmState";
export const NamedCommandValues:NamedCommand[] = [
    "SendHashes", "LeviDropdown", "DefaultCommand", "GeneralEmergency", "FSMUpdate", "SystemCheck", "ResetSenseCon", "ResetPowertrain", "ResetPropulsion", "ResetLevitation", "ResetLocalization", "Heartbeat", "FrontendHeartbeat", "EmitEvent", "StartHV", "StopHV", "LevitationOn", "LevitationOff", "PropulsionOn", "PropulsionOff", "SendPropulsionControlWord1", "SendPropulsionControlWord2", "PPControlParams", "PPDebugParams1", "PPDebugParams2", "PPTestControlParams", "PPRunParametersB", "PPRunParameters1", "PPRunParameters2", "Shutdown", "EmergencyBrake", "SystemReset", "RearmSDC", "ConnectionEstablished", "ConnectionClosed", "MockPtAck", "MockLeviAck", "MockPropAck", "MockHVOn", "RequestFsmState"];

export type NamedDatatype = "TempMotorLeft0" | "TempMotorLeft1" | "TempMotorLeft2" | "TempMotorLeft3" | "TempMotorLeft4" | "TempMotorLeft5" | "TempMotorLeft6" | "TempMotorLeft7" | "TempMotorRight0" | "TempMotorRight1" | "TempMotorRight2" | "TempMotorRight3" | "TempMotorRight4" | "TempMotorRight5" | "TempMotorRight6" | "TempMotorRight7" | "PTCState" | "PTCNonCriticalFault" | "BMSVoltageHigh" | "BMSVoltageLow" | "BMSTemperatureHigh" | "BMSTemperatureLow" | "VPack" | "IPack" | "VDCLink" | "TempRangeStart" | "TempRangeEnd" | "Localization" | "Velocity" | "PPInitFault1" | "PPInitFault2" | "PPEmergency1" | "PPEmergency2" | "Word1" | "Word2" | "IqMeasured1" | "IqReference1" | "IdMeasured1" | "IdReference1" | "IqMeasured2" | "IqReference2" | "IdMeasured2" | "IdReference2" | "Vq_Log1" | "Vd_Log1" | "Vbus1" | "Ibus1" | "CANLog" | "Vq_Log2" | "Vd_Log2" | "Vbus2" | "Ibus2" | "Ta1" | "Tb1" | "Tc1" | "TCASE1" | "Ta2" | "Tb2" | "Tc2" | "TCASE2" | "FSMAckProp1" | "FSMAckProp2" | "FSMAckLevi" | "ClearFaultAckLevi" | "Offset1" | "Offset2" | "Offset3" | "Offset4" | "LeviFault" | "LeviHeartbeat" | "LeviFSMStateChanged" | "LevitationState" | "NonCriticalLeviError" | "Vertical" | "Lateral" | "Roll" | "Pitch" | "Yaw" | "VFL1" | "VFL2" | "VFR1" | "VFR2" | "VBL1" | "VBL2" | "VBR1" | "VBR2" | "LF1" | "LF2" | "LB1" | "LB2" | "ZRequested" | "RollRequested" | "PitchRequested" | "YRequested" | "YawRequested" | "TempHEMS1" | "TempHEMS2" | "TempHEMS3" | "TempHEMS4" | "TempHEMS5" | "TempHEMS6" | "TempHEMS7" | "TempHEMS8" | "TempEMS1" | "TempEMS2" | "TempEMS3" | "TempEMS4" | "TempEMS5" | "TempEMS6" | "TempEMS7" | "TempEMS8" | "DefaultDatatype" | "CommandHash" | "DataHash" | "ConfigHash" | "ValueError" | "ValueWarning" | "ValueCausedBraking" | "LocalisationHeartbeat" | "SensorHubHeartbeat" | "FrontendHeartbeating" | "FSMState" | "FSMTransitionFail" | "Emergency" | "HVALState";

export const NamedDatatypeValues = [
    "TempMotorLeft0", "TempMotorLeft1", "TempMotorLeft2", "TempMotorLeft3", "TempMotorLeft4", "TempMotorLeft5", "TempMotorLeft6", "TempMotorLeft7", "TempMotorRight0", "TempMotorRight1", "TempMotorRight2", "TempMotorRight3", "TempMotorRight4", "TempMotorRight5", "TempMotorRight6", "TempMotorRight7", "PTCState", "PTCNonCriticalFault", "BMSVoltageHigh", "BMSVoltageLow", "BMSTemperatureHigh", "BMSTemperatureLow", "VPack", "IPack", "VDCLink", "TempRangeStart", "TempRangeEnd", "Localization", "Velocity", "PPInitFault1", "PPInitFault2", "PPEmergency1", "PPEmergency2", "Word1", "Word2", "IqMeasured1", "IqReference1", "IdMeasured1", "IdReference1", "IqMeasured2", "IqReference2", "IdMeasured2", "IdReference2", "Vq_Log1", "Vd_Log1", "Vbus1", "Ibus1", "CANLog", "Vq_Log2", "Vd_Log2", "Vbus2", "Ibus2", "Ta1", "Tb1", "Tc1", "TCASE1", "Ta2", "Tb2", "Tc2", "TCASE2", "FSMAckProp1", "FSMAckProp2", "FSMAckLevi", "ClearFaultAckLevi", "Offset1", "Offset2", "Offset3", "Offset4", "LeviFault", "LeviHeartbeat", "LeviFSMStateChanged", "LevitationState", "NonCriticalLeviError", "Vertical", "Lateral", "Roll", "Pitch", "Yaw", "VFL1", "VFL2", "VFR1", "VFR2", "VBL1", "VBL2", "VBR1", "VBR2", "LF1", "LF2", "LB1", "LB2", "ZRequested", "RollRequested", "PitchRequested", "YRequested", "YawRequested", "TempHEMS1", "TempHEMS2", "TempHEMS3", "TempHEMS4", "TempHEMS5", "TempHEMS6", "TempHEMS7", "TempHEMS8", "TempEMS1", "TempEMS2", "TempEMS3", "TempEMS4", "TempEMS5", "TempEMS6", "TempEMS7", "TempEMS8", "DefaultDatatype", "CommandHash", "DataHash", "ConfigHash", "ValueError", "ValueWarning", "ValueCausedBraking", "LocalisationHeartbeat", "SensorHubHeartbeat", "FrontendHeartbeating", "FSMState", "FSMTransitionFail", "Emergency", "HVALState"];

// Not touched by auto-gen

export type PropPoint = {
    location: number,
    iq: number,
    id: number,
}

/**
 * Datapoint to be sent to the frontend
 */
export type Datapoint = {
    datatype: NamedDatatype,
    value: number,
    timestamp: number,
    style: string,
    units: string
}

/**
 * Event channels to listen on
 */
export const EventChannel = {
    STATUS: 'status_channel',
    INFO: 'info_channel',
    WARNING: 'warning_channel',
    ERROR: 'error_channel',
}

/**
 * Function to convert data received at DATAPOINT.value to a given type
 */
export type dataConvFun<T> = (data: number, old: T) => T;

// /**
//  * BMS Module Voltage
//  * This type is to be used for store type on receiving the
//  */
// export type BmsModuleVoltage = {
//     id: bigint;
//     max: bigint;
//     min: bigint;
//     avg: bigint;
// }
//
// /**
//  * BMS Diagnostic
//  * This interface is to be used as a store type when receiving
//  */
// export type BMSDiagnostic = {
//     errors: string[]
// }
//
// /**
//  * BMS Module Temperature
//  * This type is to be used for store type on receiving bigint with the datatype
//  */
// export type BmsModuleTemperature = {
//     id: bigint;
//     max: bigint;
//     min: bigint;
//     avg: bigint;
// }
//
// /**
//  * BMS Event with attached string event
//  */
// export type BMSEvent = {
//     event: string
// }

/**
 * Log type to be displayed in the log tab
 */
export type Log = {
    log_type: LogType, message: string, timestamp: number
}

export type Procedure = {
    name: string,
    title: string,
    id: string,
    people: string[],
    equipment: string[],
    content: string
}

// export enum STATUS {
//     MAIN_PCB,
//     PROPULSION,
//     LEVITATION,
//     SENSOR_HUB,
//     LV_BATTERIES,
//     HV_BATTERIES,
//     BRAKING_PCB,
//     VOLTAGE_OVER
// }

export type LogType = 'INFO' | 'WARNING' | 'ERROR' | 'STATUS';

export const MODAL_SETTINGS: ModalSettings = {
    component: 'alertModal',
    type: 'component',
}

export const STALE_DATA_TICKS = 10;
export const VIEWPORT_HEIGHT_NORMALIZING_VALUE = 24;

export enum PTCErrorCode {
    NONE = 0x00,
    SDC_TRIGGERED = 0x01,
    IMD_TRIGGERED = 0x02,
    CONTACT_MISMATCH = 0x04,
    PRECHARGE_FAILURE = 0x08,
    FDCAN_FAILED = 0x10,
    OVERVOLTAGE = 0x20,
    BMS_FAIL = 0x40,
}