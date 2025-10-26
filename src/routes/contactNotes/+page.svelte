<script lang="ts">
    import { onMount } from "svelte";
    import {
        contactNotes,
        loadContactNotes,
        createContactNote,
        deleteContactNote,
    } from "$lib/stores/contactNotes";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import type { NoteType } from "$lib/types/enums";

    let open = false;

    // Form state
    let contactId: number = 1;
    let noteType: NoteType = "other";
    let content = "";

    onMount(loadContactNotes);

    async function handleCreate() {
        const payload = {
            contactId,
            noteType,
            content: content || undefined,
        };

        await createContactNote(payload as any);
        open = false;
        resetForm();
    }

    function resetForm() {
        contactId = 1;
        noteType = "other";
        content = "";
    }

    async function handleDelete(id: number) {
        await deleteContactNote(id);
    }
</script>

<!-- Header -->
<div class="mb-4 flex justify-between items-center">
    <h1 class="text-xl font-semibold">Notes</h1>

    <Dialog.Root bind:open>
        <Dialog.Trigger asChild><Button>New Note</Button></Dialog.Trigger>

        <Dialog.Content class="max-w-xl">
            <Dialog.Header>
                <Dialog.Title>Create Contact Note</Dialog.Title>
                <Dialog.Description>
                    Add a note associated with a specific contact or interview.
                </Dialog.Description>
            </Dialog.Header>

            <form on:submit|preventDefault={handleCreate} class="space-y-4">
                <div class="grid grid-cols-2 gap-3">
                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Contact ID</span>
                        <input
                            type="number"
                            bind:value={contactId}
                            class="rounded-md border bg-background px-3 py-2"
                            min="1"
                            required
                        />
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Note Type</span>
                        <select
                            bind:value={noteType}
                            class="rounded-md border bg-background px-3 py-2"
                            required
                        >
                            <option value="before">Before</option>
                            <option value="during">During</option>
                            <option value="after">After</option>
                            <option value="other">Other</option>
                        </select>
                    </label>
                </div>

                <label class="flex flex-col gap-1">
                    <span class="text-sm">Content</span>
                    <textarea
                        bind:value={content}
                        class="rounded-md border bg-background px-3 py-2 min-h-24"
                        placeholder="Add your notes here..."
                    />
                </label>

                <Dialog.Footer class="flex justify-end gap-2">
                    <Dialog.Close asChild>
                        <Button type="button" variant="secondary">Cancel</Button
                        >
                    </Dialog.Close>
                    <Button type="submit" variant="default">Create</Button>
                </Dialog.Footer>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<!-- Notes Table -->
<div class="overflow-x-auto rounded-lg border">
    <table class="min-w-full text-sm">
        <thead class="bg-muted/50 text-left">
            <tr>
                <th class="px-4 py-2">ID</th>
                <th class="px-4 py-2">Contact</th>
                <th class="px-4 py-2">Type</th>
                <th class="px-4 py-2">Content</th>
                <th class="px-4 py-2">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $contactNotes as n (n.id)}
                <tr class="border-t">
                    <td class="px-4 py-2">{n.id}</td>
                    <td class="px-4 py-2">{n.contactId}</td>
                    <td class="px-4 py-2">{n.noteType}</td>
                    <td class="px-4 py-2 truncate max-w-xs">{n.content}</td>
                    <td class="px-4 py-2">
                        <button
                            class="rounded-md border px-2 py-1 hover:bg-destructive hover:text-destructive-foreground"
                            on:click={() => handleDelete(n.id)}
                        >
                            Delete
                        </button>
                    </td>
                </tr>
            {/each}
            {#if $contactNotes.length === 0}
                <tr class="border-t">
                    <td
                        colspan="5"
                        class="text-center py-8 text-muted-foreground"
                    >
                        No notes yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
