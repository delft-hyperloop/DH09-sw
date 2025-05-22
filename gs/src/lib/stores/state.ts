import { writable, type Writable } from 'svelte/store';
import {PlotBuffer} from "$lib";

export enum ErrorStatus {
    SAFE,
    WARNING,
    UNSAFE,
}

export const detailsPanelTab: Writable<string> = writable("Home");
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

export const goingForward: Writable<boolean> = writable(true);

export const RedHVALTurnedOn: Writable<boolean> = writable(false);
export const GreenHVALTurnedOn: Writable<boolean> = writable(false);

// Showcase states
export const showcaseStateCounter: Writable<number> = writable(0);
export const showcasingStates: Writable<boolean> = writable(false);
export const showcasingLocalization: Writable<boolean> = writable(false); // TODO

export const debugModeActive: Writable<boolean> = writable(false);

export const propControlWord1: Writable<number> = writable<number>(0);
export const propControlWord2: Writable<number> = writable<number>(0);

export const propulsionConfigSent: Writable<boolean> = writable(false);

// FSM states
// export const FSMState: Writable<number> = writable(0);
export const inStateSystemCheck: Writable<boolean> = writable(false);
export const inStateIdle: Writable<boolean> = writable(false); // TODO: for warnings outside of intended state
export const inStateLevitating: Writable<boolean> = writable(false); // TODO: for warnings outside of intended state
export const inStateHVOn: Writable<boolean> = writable(false); // TODO: for warnings outside of intended state

export const overrideDependencies: Writable<boolean> = writable(false);
export const usingTestTrack: Writable<boolean> = writable(true);

// Modal states
export const displayModal: Writable<boolean> = writable(false);

// 3D mode states
export const threeDModeActive: Writable<boolean> = writable(false);
export const enteringScene: Writable<boolean> = writable(false);
export const inScene: Writable<boolean> = writable(false);
export const menuOpen: Writable<boolean> = writable(false);
