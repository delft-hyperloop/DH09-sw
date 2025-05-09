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

export const chartStore = writable(new Map<string, PlotBuffer>());

export const serverStatus: Writable<boolean> = writable(false);
export const bigErrorStatus: Writable<ErrorStatus> = writable(ErrorStatus.SAFE);
export const latestTimestamp: Writable<number> = writable(0);

export const logsVisible: Writable<boolean> = writable(false);

export const goingForward: Writable<boolean> = writable(true);

export const RedHVALTurnedOn: Writable<boolean> = writable(false);
export const GreenHVALTurnedOn: Writable<boolean> = writable(false);

export const showcaseStateCounter: Writable<number> = writable(0);
export const showcasingStates: Writable<boolean> = writable(false);
export const showcasingLocalization: Writable<boolean> = writable(false);

export const debugModeActive: Writable<boolean> = writable(false);

export const propControlWord1: Writable<number> = writable<number>(0);
export const propControlWord2: Writable<number> = writable<number>(0);

export const propulsionConfigSent: Writable<boolean> = writable(false);
export const inStateSystemCheck: Writable<boolean> = writable(false);
export const inIdleState: Writable<boolean> = writable(false);
export const hvOn: Writable<boolean> = writable(false);

export const overrideDependencies: Writable<boolean> = writable(false);
export const usingTestTrack: Writable<boolean> = writable(true);
