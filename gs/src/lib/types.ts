import type { ModalSettings } from '@skeletonlabs/skeleton';

/* BEGIN AUTO GENERATED TYPES */
export type NamedCommand = "SendHashes" | "LeviDropdown" | "DefaultCommand" | "GeneralEmergency" | "FSMUpdate" | "SystemCheck" | "ResetSenseCon" | "ResetPowertrain" | "ResetPropulsion" | "ResetLevitation" | "ResetLocalization" | "Heartbeat" | "FrontendHeartbeat" | "EmitEvent" | "StartHV" | "StopHV" | "LevitationOn" | "LevitationOff" | "PropulsionOn" | "MotorBrake" | "SendPropulsionControlWord1" | "SendPropulsionControlWord2" | "PPControlParams" | "PPDebugParams1" | "PPDebugParams2" | "PPTestControlParams" | "PPRunParametersB" | "PPRunParameters1" | "PPRunParameters2" | "Shutdown" | "EmergencyBrake" | "SystemReset" | "RearmSDC" | "ConnectionEstablished" | "ConnectionClosed" | "MockLeviAck" | "MockProp1Ack" | "MockProp2Ack" | "MockHVOn" | "Charge" | "StopCharge" | "FaultFixed" | "FailLeviSystemCheck" | "FailProp1SystemCheck" | "FailProp2SystemCheck" | "ReconnectEmergency" | "OverrideRearmSdc";
export const NamedCommandValues:NamedCommand[] = [
"SendHashes", "LeviDropdown", "DefaultCommand", "GeneralEmergency", "FSMUpdate", "SystemCheck", "ResetSenseCon", "ResetPowertrain", "ResetPropulsion", "ResetLevitation", "ResetLocalization", "Heartbeat", "FrontendHeartbeat", "EmitEvent", "StartHV", "StopHV", "LevitationOn", "LevitationOff", "PropulsionOn", "MotorBrake", "SendPropulsionControlWord1", "SendPropulsionControlWord2", "PPControlParams", "PPDebugParams1", "PPDebugParams2", "PPTestControlParams", "PPRunParametersB", "PPRunParameters1", "PPRunParameters2", "Shutdown", "EmergencyBrake", "SystemReset", "RearmSDC", "ConnectionEstablished", "ConnectionClosed", "MockLeviAck", "MockProp1Ack", "MockProp2Ack", "MockHVOn", "Charge", "StopCharge", "FaultFixed", "FailLeviSystemCheck", "FailProp1SystemCheck", "FailProp2SystemCheck", "ReconnectEmergency", "OverrideRearmSdc"];

export type NamedDatatype = "TempMotorLeft0" | "TempMotorLeft1" | "TempMotorLeft2" | "TempMotorLeft3" | "TempMotorLeft4" | "TempMotorLeft5" | "TempMotorLeft6" | "TempMotorLeft7" | "TempMotorRight0" | "TempMotorRight1" | "TempMotorRight2" | "TempMotorRight3" | "TempMotorRight4" | "TempMotorRight5" | "TempMotorRight6" | "TempMotorRight7" | "PTCState" | "HVALState" | "IMDWarnings" | "PTCErrors" | "HvVHigh" | "HvVLow" | "BMSTemperatureHigh" | "BMSTemperatureLow" | "VPack" | "IPack" | "VDCLink" | "TempRangeStart" | "TempRangeEnd" | "Localization" | "Velocity" | "PPInitFault1" | "PPInitFault2" | "PPEmergency1" | "PPEmergency2" | "Word1" | "Word2" | "IqMeasured1" | "IqReference1" | "IdMeasured1" | "IdReference1" | "IqMeasured2" | "IqReference2" | "IdMeasured2" | "IdReference2" | "Vq_Log1" | "Vd_Log1" | "Vbus1" | "Ibus1" | "CANLog" | "Vq_Log2" | "Vd_Log2" | "Vbus2" | "Ibus2" | "Ta1" | "Tb1" | "Tc1" | "TCASE1" | "Ta2" | "Tb2" | "Tc2" | "TCASE2" | "FSMAckProp1" | "FSMAckProp2" | "FSMAckLevi" | "ClearFaultAckLevi" | "Offset1" | "Offset2" | "Offset3" | "Offset4" | "LeviSystemCheckResponse" | "LvVHigh" | "LvVLow" | "THigh" | "TLow" | "IsolationResistance" | "BusCurrent" | "VPackLowVoltage" | "IPackLowVoltage" | "PtSystemCheckResponse" | "LeviFault" | "LeviFaultDriveNumber" | "LeviHeartbeat" | "LeviFSMStateChanged" | "LevitationState" | "NonCriticalLeviError" | "Vertical" | "Lateral" | "Roll" | "Pitch" | "Yaw" | "VFL1" | "VFL2" | "VFR1" | "VFR2" | "VBL1" | "VBL2" | "VBR1" | "VBR2" | "LF1" | "LF2" | "LB1" | "LB2" | "ZRequested" | "RollRequested" | "PitchRequested" | "YRequested" | "YawRequested" | "TempHEMS1" | "TempHEMS2" | "TempHEMS3" | "TempHEMS4" | "TempHEMS5" | "TempHEMS6" | "TempHEMS7" | "TempHEMS8" | "TempEMS1" | "TempEMS2" | "TempEMS3" | "TempEMS4" | "TempEMS5" | "TempEMS6" | "TempEMS7" | "TempEMS8" | "PressureLow" | "PressureHigh" | "SensorHubEmergency" | "PtcErrorEmergency" | "BmsErrorLowVoltage" | "BmsErrorHighVoltage" | "DefaultDatatype" | "CommandHash" | "DataHash" | "ConfigHash" | "ValueError" | "ValueWarning" | "ValueCausedBraking" | "LocalisationHeartbeat" | "SensorHubHeartbeat" | "FrontendHeartbeating" | "FSMState" | "FSMTransitionFail" | "Emergency" | "LeviSystemCheckSuccess" | "LeviSystemCheckFailure" | "Prop1SystemCheckSuccess" | "Prop1SystemCheckFailure" | "Prop2SystemCheckSuccess" | "Prop2SystemCheckFailure" | "ResetFSM" | "EmergencyStaleCriticalData" | "LocalizationLimitReached";

export const NamedDatatypeValues = [
"TempMotorLeft0", "TempMotorLeft1", "TempMotorLeft2", "TempMotorLeft3", "TempMotorLeft4", "TempMotorLeft5", "TempMotorLeft6", "TempMotorLeft7", "TempMotorRight0", "TempMotorRight1", "TempMotorRight2", "TempMotorRight3", "TempMotorRight4", "TempMotorRight5", "TempMotorRight6", "TempMotorRight7", "PTCState", "HVALState", "IMDWarnings", "PTCErrors", "HvVHigh", "HvVLow", "BMSTemperatureHigh", "BMSTemperatureLow", "VPack", "IPack", "VDCLink", "TempRangeStart", "TempRangeEnd", "Localization", "Velocity", "PPInitFault1", "PPInitFault2", "PPEmergency1", "PPEmergency2", "Word1", "Word2", "IqMeasured1", "IqReference1", "IdMeasured1", "IdReference1", "IqMeasured2", "IqReference2", "IdMeasured2", "IdReference2", "Vq_Log1", "Vd_Log1", "Vbus1", "Ibus1", "CANLog", "Vq_Log2", "Vd_Log2", "Vbus2", "Ibus2", "Ta1", "Tb1", "Tc1", "TCASE1", "Ta2", "Tb2", "Tc2", "TCASE2", "FSMAckProp1", "FSMAckProp2", "FSMAckLevi", "ClearFaultAckLevi", "Offset1", "Offset2", "Offset3", "Offset4", "LeviSystemCheckResponse", "LvVHigh", "LvVLow", "THigh", "TLow", "IsolationResistance", "BusCurrent", "VPackLowVoltage", "IPackLowVoltage", "PtSystemCheckResponse", "LeviFault", "LeviFaultDriveNumber", "LeviHeartbeat", "LeviFSMStateChanged", "LevitationState", "NonCriticalLeviError", "Vertical", "Lateral", "Roll", "Pitch", "Yaw", "VFL1", "VFL2", "VFR1", "VFR2", "VBL1", "VBL2", "VBR1", "VBR2", "LF1", "LF2", "LB1", "LB2", "ZRequested", "RollRequested", "PitchRequested", "YRequested", "YawRequested", "TempHEMS1", "TempHEMS2", "TempHEMS3", "TempHEMS4", "TempHEMS5", "TempHEMS6", "TempHEMS7", "TempHEMS8", "TempEMS1", "TempEMS2", "TempEMS3", "TempEMS4", "TempEMS5", "TempEMS6", "TempEMS7", "TempEMS8", "PressureLow", "PressureHigh", "SensorHubEmergency", "PtcErrorEmergency", "BmsErrorLowVoltage", "BmsErrorHighVoltage", "DefaultDatatype", "CommandHash", "DataHash", "ConfigHash", "ValueError", "ValueWarning", "ValueCausedBraking", "LocalisationHeartbeat", "SensorHubHeartbeat", "FrontendHeartbeating", "FSMState", "FSMTransitionFail", "Emergency", "LeviSystemCheckSuccess", "LeviSystemCheckFailure", "Prop1SystemCheckSuccess", "Prop1SystemCheckFailure", "Prop2SystemCheckSuccess", "Prop2SystemCheckFailure", "ResetFSM", "EmergencyStaleCriticalData", "LocalizationLimitReached"];
/* END AUTO GENERATED TYPES */

// Not touched by auto-gen

export type PropPoint = {
    location: number;
    // iq: number,
    // id: number,
    imax: number;
};

/**
 * Datapoint to be sent to the frontend
 */
export type Datapoint = {
    datatype: NamedDatatype;
    value: number;
    timestamp: number;
    style: string;
    units: string;
    upper: number | undefined;
    lower: number | undefined;
};

/**
 * Event channels to listen on
 */
export const EventChannel = {
    STATUS: 'status_channel',
    INFO: 'info_channel',
    WARNING: 'warning_channel',
    ERROR: 'error_channel',
};

/**
 * Function to convert data received at DATAPOINT.value to a given type
 */
export type dataConvFun<T> = (data: number, old: T) => T;

/**
 * Type for the FSM states
 */
export type FsmState = {
    element: SVGGElement | null;
    index: number;
};

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
    log_type: LogType;
    message: string;
    timestamp: number;
};

export type Procedure = {
    name: string;
    title: string;
    id: string;
    people: string[];
    equipment: string[];
    content: string;
};

export type LogType = 'INFO' | 'WARNING' | 'ERROR' | 'STATUS' | 'TRACE';

export const MODAL_SETTINGS: ModalSettings = {
    component: 'alertModal',
    type: 'component',
};

export const ptcErrorCodes = [
    'None',
    'SDC Triggered',
    'IMD Triggered',
    'Contact Mismatch',
    'Precharge Failure',
    'FDCAN Failed',
    'Overvoltage',
    'BMS Fail',
];

export const ptcStates = ['Idle', 'Precharge', 'HV On', 'Failure', 'Discharge'];

export const imdWarnings = [
    'None',
    'Device Error Active',
    'HV_pos Connection Failure',
    'HV_min Connection Failure',
    'Earth Connection Failure',
    'Iso Alarm',
    'Iso Warning',
    'Iso Outdated',
    'Unbalance Error',
    'Undervoltage Alarm',
    'Unsafe to Start',
    'Earthlift Open',
];

export const senorHubEmergencies = [
    'Localization Timeout/Disconnect',
    'Temperature Timeout/Disconnect',
    'Pressure Leak',
];

export const bmsErrors = [
    'OVP',
    'UVP',
    'OTP',
    'OCP',
    'OVP_trip',
    'UVP_trip',
    'OTP_trip',
    'OCP_trip',
];

export enum EBSStates {
    Triggered = 'Triggered',
    Armed = 'Armed',
}

export const leviErrorMessages = [
    'Resettable Drive Error',
    'Non-Resettable Drive Error',
    'DC Link Undervoltage',
    'Lase Offset Sensor Disconnected',
    'Drive Current Following Error',
    'Levi Error because SenseCon went into Fault',
    'Drive Error Reset Failed',
    'Other',
];