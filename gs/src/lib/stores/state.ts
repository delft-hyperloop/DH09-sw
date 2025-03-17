import { writable, type Writable } from 'svelte/store';
import {PlotBuffer} from "$lib";

export enum ErrorStatus {
  SAFE,
  WARNING,
  UNSAFE,
}

export const detailTabSet: Writable<number> = writable(0);
export const inputSpeed: Writable<number> = writable(50);
export const inputEmerg: Writable<number> = writable(-1);

export const vitals_pane: Writable<number> = writable(40)
export const details_pane: Writable<number> = writable(80)

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

export const debugModeActive: Writable<boolean> = writable(false);