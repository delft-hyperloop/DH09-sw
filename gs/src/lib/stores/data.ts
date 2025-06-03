// src/lib/stores/data.ts

import type {Procedure} from "$lib/types";
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

export const podTargetSpeed: Writable<number> = writable(0);
export const propMaxPower: Writable<number> = writable(0);

export const modalTitle: Writable<string> = writable("");
export const modalBody: Writable<string> = writable("");

export const lastHeartbeatTimestamp: Writable<number> = writable(Date.now());
