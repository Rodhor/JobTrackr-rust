import { writable } from "svelte/store";

export const callerStack = writable<string[]>([]);

export function pushCaller(caller: string) {
  callerStack.update((stack) => [...stack, caller]);
}

export function popCaller() {
  let nextCaller: string | undefined;
  callerStack.update((stack) => {
    nextCaller = stack.pop();
    return stack;
  });
  return nextCaller;
}

export function resetCallerStack() {
  callerStack.set([]);
}
