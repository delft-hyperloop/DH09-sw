/* AUTO GENERATED USING npm run generate:gs */
export type NamedCommand = "DefaultCommand" | "Heartbeat" | "FrontendHeartbeat" | "EmitEvent" | "StartHV" | "StopHV" | "LevitationOn" | "LevitationOff" | "vertical_0_current" | "vert_0_current_reset" | "PropulsionOn" | "PropulsionOff" | "SendPropulsionControlWord" | "PPControlParams" | "PPDebugParams11" | "PPDebugParams12" | "PPDebugParams2" | "PPTestControlParams1" | "PPTestControlParams2" | "SubmitDirection" | "SubmitSpeed" | "ArmBrakes" | "Shutdown" | "EmergencyBrake" | "SystemReset";
export const NamedCommandValues: NamedCommand[] = [
    "DefaultCommand", "Heartbeat", "FrontendHeartbeat", "EmitEvent", "StartHV", "StopHV", "LevitationOn", "LevitationOff", "vertical_0_current", "vert_0_current_reset", "PropulsionOn", "PropulsionOff", "SendPropulsionControlWord", "PPControlParams", "PPDebugParams11", "PPDebugParams12", "PPDebugParams2", "PPTestControlParams1", "PPTestControlParams2", "SubmitDirection", "SubmitSpeed", "ArmBrakes", "Shutdown", "EmergencyBrake", "SystemReset"];

export type NamedDatatype = "TempAmbient0" | "TempAmbient1" | "TempAmbient2" | "BMSVoltageHigh" | "BMSVoltageLow" | "BMSTemperatureHigh" | "BMSTemperatureLow" | "TempRangeStart" | "TempRangeEnd" | "Loc1" | "Loc2" | "Temp0" | "Temp1" | "Temp2" | "Temp3" | "Temp4" | "Temp5" | "Temp6" | "Temp7" | "ResetSenseCon" | "ResetPropulsion" | "ModulationFactor1" | "ModulationFactor2" | "MaximumVelocity1" | "MaximumVelocity2" | "Kpq1" | "Kpq2" | "Kiq1" | "Kiq2" | "Kpd1" | "Kpd2" | "Kid1" | "Kid2" | "PositionOffset1" | "PositionOffset2" | "Alpha1" | "Alpha2" | "Iq1" | "Iq2" | "Id1" | "Id2" | "Vq1_C" | "Vq2_C" | "Vd1_C" | "Vd2_C" | "PPEmergency" | "Word1" | "Word2" | "IqMeasured" | "IqReference" | "IdMeasured" | "IdReference" | "Vq_Log" | "Vd_Log" | "Vbus" | "Ibus" | "Ta" | "Tb" | "Tc" | "TCASE" | "DefaultDatatype" | "CommandHash" | "EventsHash" | "DataHash" | "ConfigHash" | "ValueError" | "ValueWarning" | "ValueCausedBraking" | "LocalisationHeartbeat" | "SensorHubHeartbeat" | "FrontendHeartbeating" | "PropulsionCurrent" | "Localisation";

export const NamedDatatypeValues = [
    "TempAmbient0", "TempAmbient1", "TempAmbient2", "BMSVoltageHigh", "BMSVoltageLow", "BMSTemperatureHigh", "BMSTemperatureLow", "TempRangeStart", "TempRangeEnd", "Loc1", "Loc2", "Temp0", "Temp1", "Temp2", "Temp3", "Temp4", "Temp5", "Temp6", "Temp7", "ResetSenseCon", "ResetPropulsion", "ModulationFactor1", "ModulationFactor2", "MaximumVelocity1", "MaximumVelocity2", "Kpq1", "Kpq2", "Kiq1", "Kiq2", "Kpd1", "Kpd2", "Kid1", "Kid2", "PositionOffset1", "PositionOffset2", "Alpha1", "Alpha2", "Iq1", "Iq2", "Id1", "Id2", "Vq1_C", "Vq2_C", "Vd1_C", "Vd2_C", "PPEmergency", "Word1", "Word2", "IqMeasured", "IqReference", "IdMeasured", "IdReference", "Vq_Log", "Vd_Log", "Vbus", "Ibus", "Ta", "Tb", "Tc", "TCASE", "DefaultDatatype", "CommandHash", "EventsHash", "DataHash", "ConfigHash", "ValueError", "ValueWarning", "ValueCausedBraking", "LocalisationHeartbeat", "SensorHubHeartbeat", "FrontendHeartbeating", "PropulsionCurrent", "Localisation"];
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

/**
 * BMS Module Voltage
 * This type is to be used for store type on receiving the
 */
export type BmsModuleVoltage = {
    id: bigint;
    max: bigint;
    min: bigint;
    avg: bigint;
}

/**
 * BMS Diagnostic
 * This interface is to be used as a store type when receiving
 */
export type BMSDiagnostic = {
    errors: string[]
}

/**
 * BMS Module Temperature
 * This type is to be used for store type on receiving bigint with the datatype
 */
export type BmsModuleTemperature = {
    id: bigint;
    max: bigint;
    min: bigint;
    avg: bigint;
}

/**
 * BMS Event with attached string event
 */
export type BMSEvent = {
    event: string
}

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

export enum STATUS {
    MAIN_PCB,
    PROPULSION,
    LEVITATION,
    SENSOR_HUB,
    LV_BATTERIES,
    HV_BATTERIES,
    BRAKING_PCB,
    VOLTAGE_OVER
}

export type LogType = 'INFO' | 'WARNING' | 'ERROR' | 'STATUS';

export type RouteConfig = {
    speed: number,
    current_position: number
}

export const LOCALISATION_NAME = "Localisation"; // 'levi_location'
export const GOING_FORWARD = "GoingForward";
