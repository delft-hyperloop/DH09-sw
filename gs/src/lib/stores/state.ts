import { writable, type Writable } from 'svelte/store';
import {PlotBuffer} from "$lib";

export enum ErrorStatus {
  SAFE,
  WARNING,
  UNSAFE,
}

export const detailTabSet: Writable<number> = writable(0);
export const inputEmerg: Writable<number> = writable(-1);

export const logsPanelSize: Writable<number> = writable(5);
export const logsScrollAreaSize: Writable<number> = writable(0);
export const logsVisible: Writable<boolean> = writable(false);

export const chartStore = writable(new Map<string, PlotBuffer>());
export const propChartStore: Writable<string[]> = writable([]);
export const leviChartStore: Writable<string[]> = writable([]);
export const powertrainChartStore: Writable<string[]> = writable([]);

export const serverStatus: Writable<boolean> = writable(false);
export const bigErrorStatus: Writable<ErrorStatus> = writable(ErrorStatus.SAFE);
export const latestTimestamp: Writable<number> = writable(0);
export const connectedToMainPCB: Writable<boolean> = writable(false);

export const goingForward: Writable<boolean> = writable(true);

export const RedHVALTurnedOn: Writable<boolean> = writable(false);
export const GreenHVALTurnedOn: Writable<boolean> = writable(false);

export const showcaseStateCounter: Writable<number> = writable(0);
export const showcasingStates: Writable<boolean> = writable(false);
export const showcasingLocalization: Writable<boolean> = writable(false); // TODO

export const debugModeActive: Writable<boolean> = writable(false);
export const threeDModeActive: Writable<boolean> = writable(false);

export const propControlWord1: Writable<number> = writable<number>(0);
export const propControlWord2: Writable<number> = writable<number>(0);

export const propulsionConfigSent: Writable<boolean> = writable(false);
// export const inStateSystemCheck: Writable<boolean> = writable(false);
export const inStateIdle: Writable<boolean> = writable(false);
export const inStateActive: Writable<boolean> = writable(false);
export const inStateLevitating: Writable<boolean> = writable(false);
export const inStateAccelerating: Writable<boolean> = writable(false);
export const inStateDemo: Writable<boolean> = writable(false);
export const inStateConnectedToGS: Writable<boolean> = writable(false);

export const overrideDependencies: Writable<boolean> = writable(false);
export const usingTestTrack: Writable<boolean> = writable(true);

export const leftMotorTempsAcknowledged: Writable<boolean> = writable(true);
export const rightMotorTempsAcknowledged: Writable<boolean> = writable(true);
export const emsTempsAcknowledged: Writable<boolean> = writable(true);
export const hemsTempsAcknowledged: Writable<boolean> = writable(true);
export const propInitFault1Acknowledged: Writable<boolean> = writable(true);
export const propInitFault2Acknowledged: Writable<boolean> = writable(true);
export const propEmergency1Acknowledged: Writable<boolean> = writable(true);
export const propEmergency2Acknowledged: Writable<boolean> = writable(true);
export const connectionAcknowledged: Writable<boolean> = writable(true);

export const inDropdown: Writable<boolean> = writable(false);
