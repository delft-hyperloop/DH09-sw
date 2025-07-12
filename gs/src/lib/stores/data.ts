// src/lib/stores/data.ts

import type { NamedCommand, Procedure, PropPoint } from '$lib/types';
import {writable, type Writable} from 'svelte/store';
import {invoke} from "@tauri-apps/api/tauri";
import {parseProcedure} from "$lib/util/parsers";

export const procedures: Writable<Procedure[]> = writable([{
  name: "NO PROCEDURES LOADED",
  title: "NO PROCEDURES LOADED",
  people: ["NO PROCEDURES LOADED"],
  content: "NO PROCEDURES LOADED",
  equipment: ["NO PROCEDURES LOADED"],
  id: "NO PROCEDURES LOADED"
}]);

export const initProcedures = async () => {
  await invoke("procedures").then(r => {
    console.log(`PROCEDURES LOADED`);
    procedures.set((r as string[][]).map(parseProcedure));
  }).catch((e) => {
    console.error(`Error loading procedures: ${e}`);
  });
}

export const pinnedCharts: Writable<Array<string>> = writable([]);
export const displayedCharts: Writable<Array<string>> = writable([]);

export const modalTitle: Writable<string> = writable("");
export const modalBody: Writable<string> = writable("");

export const lastHeartbeatTimestamp: Writable<number> = writable(Date.now());

export const propulsionPoints: Writable<PropPoint[]> = writable([
  {
    location: 0,
    imax: 0,
  },
  {
    location: 0,
    imax: 0,
  },
  {
    location: 0,
    imax: 0,
  }
]);
export const setPropulsionPoints: Writable<PropPoint[]> = writable([
  {
    location: 0,
    imax: 0,
  },
  {
    location: 0,
    imax: 0,
  },
  {
    location: 0,
    imax: 0,
  }
]);

export const staleCriticalDatatypes: Writable<string[]> = writable([]);

export const emergencySources: Writable<string[]> = writable([]);
export const nextStateMessage: Writable<string> = writable("explanation");
export const nextRecommendedStateCmd: Writable<NamedCommand> = writable("DefaultCommand");
