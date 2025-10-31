import { writable } from "svelte/store";

export function createDefaultForm<T extends Record<string, any>>(defaults: T) {
  const formData = writable<T>({ ...defaults });

  function resetForm(data?: Partial<T>) {
    formData.set({ ...defaults, ...data });
  }

  return { formData, resetForm };
}
