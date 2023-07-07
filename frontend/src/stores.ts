import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import { browser } from "$app/environment";

// Generic store functions
function setSessionStore<T>(key: string, value: T): void {
  sessionStorage.setItem(key, JSON.stringify(value));
}

function getSessionStore<T>(key: string): T | null {
  return JSON.parse(sessionStorage.getItem(key) || "null") as T;
}

// A generic writable store that persists to sessionStorage
export function writableSession<T>(key: string, value: T): Writable<T> {
  if (!browser) return writable(value); // Mock for SSR
  const sessionValue = getSessionStore<T>(key);
  if (!sessionValue) setSessionStore(key, value);

  const store = writable(sessionValue || value);
  store.subscribe(value => {
    setSessionStore(key, value);
  });

  return store;
}
//  END Generic store functions

// make a writableSession if we have a browser
const emptyQuery = {
  block: [],
  study_level: [],
  schedule_group: [],
  examination_type: [],
  searches: [],
};

export const queryStore = writableSession("query", emptyQuery);

export function clearAll() {
  // Cause the checkboxes to update
  queryStore.update((store) => {
    store.block = [];
    store.study_level = [];
    store.schedule_group = [];
    store.examination_type = [];
    store.searches = [];
    return store;
  });
}

// API URL
export function apiUrl() {
  // check NODE_ENV
  // check that window is defined
  if (typeof window === "undefined") {
    return "https://disku.jniemela.dk/api"; // SSR
  }

  let hostname = window.location.hostname;
  if (hostname == "localhost") {
    return "http://localhost:3000/api";
  }

  // if running on another host, assume we are in prod
  return "https://" + hostname + "/api";
}
