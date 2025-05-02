import type { ModalSettings } from '@skeletonlabs/skeleton';

/* AUTO GENERATED USING npm run generate:gs */
export type NamedCommand = "DefaultCommand" | "GeneralEmergency" | "FSMUpdate" | "SystemCheck" | "ResetSenseCon" | "ResetPowertrain" | "ResetPropulsion" | "ResetLevitation" | "ResetLocalization" | "Heartbeat" | "FrontendHeartbeat" | "EmitEvent" | "StartHV" | "StopHV" | "LevitationOn" | "LevitationOff" | "vertical_0_current" | "vert_0_current_reset" | "PropulsionOn" | "PropulsionOff" | "SendPropulsionControlWord1" | "SendPropulsionControlWord2" | "PPControlParams" | "PPDebugParams1" | "PPDebugParams2" | "PPTestControlParams" | "ArmBrakes" | "Shutdown" | "EmergencyBrake" | "SystemReset";
export const NamedCommandValues:NamedCommand[] = [
    "DefaultCommand", "GeneralEmergency", "FSMUpdate", "SystemCheck", "ResetSenseCon", "ResetPowertrain", "ResetPropulsion", "ResetLevitation", "ResetLocalization", "Heartbeat", "FrontendHeartbeat", "EmitEvent", "StartHV", "StopHV", "LevitationOn", "LevitationOff", "vertical_0_current", "vert_0_current_reset", "PropulsionOn", "PropulsionOff", "SendPropulsionControlWord1", "SendPropulsionControlWord2", "PPControlParams", "PPDebugParams1", "PPDebugParams2", "PPTestControlParams", "ArmBrakes", "Shutdown", "EmergencyBrake", "SystemReset"];

export type NamedDatatype = "TempMotorLeft0" | "TempMotorLeft1" | "TempMotorLeft2" | "TempMotorLeft3" | "TempMotorLeft4" | "TempMotorLeft5" | "TempMotorRight0" | "TempMotorRight1" | "TempMotorRight2" | "TempMotorRight3" | "TempMotorRight4" | "TempMotorRight5" | "TempMotorRight6" | "PTCState" | "PTCNonCriticalFault" | "BMSVoltageHigh" | "BMSVoltageLow" | "BMSTemperatureHigh" | "BMSTemperatureLow" | "VPack" | "IPack" | "VDCLink" | "TempRangeStart" | "TempRangeEnd" | "Localization" | "Temp0" | "Temp1" | "Temp2" | "Temp3" | "Temp4" | "Temp5" | "Temp6" | "Temp7" | "ModulationFactor1" | "ModulationFactor2" | "MaximumVelocity1" | "MaximumVelocity2" | "Kpq1" | "Kpq2" | "Kiq1" | "Kiq2" | "Kpd1" | "Kpd2" | "Kid1" | "Kid2" | "PositionOffset1" | "PositionOffset2" | "Alpha1" | "Alpha2" | "Iq1" | "Iq2" | "Id1" | "Id2" | "Vq1_C" | "Vq2_C" | "Vd1_C" | "Vd2_C" | "PPInitFault1" | "PPInitFault2" | "PPEmergency1" | "PPEmergency2" | "Word1" | "Word2" | "IqMeasured1" | "IqReference1" | "IdMeasured1" | "IdReference1" | "IqMeasured2" | "IqReference2" | "IdMeasured2" | "IdReference2" | "Vq_Log1" | "Vd_Log1" | "Vbus1" | "Ibus1" | "CANLog" | "Vq_Log2" | "Vd_Log2" | "Vbus2" | "Ibus2" | "Ta1" | "Tb1" | "Tc1" | "TCASE1" | "Ta2" | "Tb2" | "Tc2" | "TCASE2" | "FSMAckProp1" | "FSMAckProp2" | "DefaultDatatype" | "CommandHash" | "EventsHash" | "DataHash" | "ConfigHash" | "ValueError" | "ValueWarning" | "ValueCausedBraking" | "LocalisationHeartbeat" | "SensorHubHeartbeat" | "FrontendHeartbeating" | "FSMState";

export const NamedDatatypeValues = [
    "TempMotorLeft0", "TempMotorLeft1", "TempMotorLeft2", "TempMotorLeft3", "TempMotorLeft4", "TempMotorLeft5", "TempMotorRight0", "TempMotorRight1", "TempMotorRight2", "TempMotorRight3", "TempMotorRight4", "TempMotorRight5", "TempMotorRight6", "PTCState", "PTCNonCriticalFault", "BMSVoltageHigh", "BMSVoltageLow", "BMSTemperatureHigh", "BMSTemperatureLow", "VPack", "IPack", "VDCLink", "TempRangeStart", "TempRangeEnd", "Localization", "Temp0", "Temp1", "Temp2", "Temp3", "Temp4", "Temp5", "Temp6", "Temp7", "ModulationFactor1", "ModulationFactor2", "MaximumVelocity1", "MaximumVelocity2", "Kpq1", "Kpq2", "Kiq1", "Kiq2", "Kpd1", "Kpd2", "Kid1", "Kid2", "PositionOffset1", "PositionOffset2", "Alpha1", "Alpha2", "Iq1", "Iq2", "Id1", "Id2", "Vq1_C", "Vq2_C", "Vd1_C", "Vd2_C", "PPInitFault1", "PPInitFault2", "PPEmergency1", "PPEmergency2", "Word1", "Word2", "IqMeasured1", "IqReference1", "IdMeasured1", "IdReference1", "IqMeasured2", "IqReference2", "IdMeasured2", "IdReference2", "Vq_Log1", "Vd_Log1", "Vbus1", "Ibus1", "CANLog", "Vq_Log2", "Vd_Log2", "Vbus2", "Ibus2", "Ta1", "Tb1", "Tc1", "TCASE1", "Ta2", "Tb2", "Tc2", "TCASE2", "FSMAckProp1", "FSMAckProp2", "DefaultDatatype", "CommandHash", "EventsHash", "DataHash", "ConfigHash", "ValueError", "ValueWarning", "ValueCausedBraking", "LocalisationHeartbeat", "SensorHubHeartbeat", "FrontendHeartbeating", "FSMState"];

// Not touched by auto-gen

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