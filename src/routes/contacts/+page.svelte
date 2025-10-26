<script lang="ts">
    import { onMount } from "svelte";
    import {
        contacts,
        loadContacts,
        createContact,
        deleteContact,
    } from "$lib/stores/contacts";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import type { ContactType } from "$lib/types/enums";

    let open = false;

    // Form fields for creating new contact
    let contactType: ContactType = "email";
    let userId: number = 1;
    let personId: number | null = null;
    let location = "";
    let contactDate = new Date().toISOString();

    onMount(loadContacts);

    async function handleCreate() {
        const payload = {
            contactType,
            contactDate,
            location: location || undefined,
            userId,
            personId: personId || undefined,
        };

        await createContact(payload as any);
        open = false;
        resetForm();
    }

    function resetForm() {
        contactType = "email";
        location = "";
        personId = null;
        contactDate = new Date().toISOString();
    }

    async function handleDelete(id: number) {
        await deleteContact(id);
    }
</script>

<!-- Header + Modal Trigger -->
<div class="mb-4 flex justify-between items-center">
    <h1 class="text-xl font-semibold">Interactions</h1>

    <Dialog.Root bind:open>
        <Dialog.Trigger asChild><Button>New Interaction</Button></Dialog.Trigger
        >

        <Dialog.Content class="max-w-xl">
            <Dialog.Header>
                <Dialog.Title>Create Interaction</Dialog.Title>
                <Dialog.Description>
                    Add a new contact record — interview, phone call, or
                    meeting.
                </Dialog.Description>
            </Dialog.Header>

            <form on:submit|preventDefault={handleCreate} class="space-y-4">
                <div class="grid grid-cols-2 gap-3">
                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Contact Type</span>
                        <select
                            bind:value={contactType}
                            class="rounded-md border bg-background px-3 py-2"
                            required
                        >
                            <option value="phone">Phone</option>
                            <option value="email">Email</option>
                            <option value="in_person">In Person</option>
                            <option value="other">Other</option>
                        </select>
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Contact Date</span>
                        <input
                            type="text"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={contactDate}
                            required
                        />
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">User ID</span>
                        <input
                            type="number"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={userId}
                            min="1"
                            required
                        />
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Person ID</span>
                        <input
                            type="number"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={personId}
                            min="1"
                            placeholder="Optional"
                        />
                    </label>
                </div>

                <label class="flex flex-col gap-1">
                    <span class="text-sm">Location</span>
                    <input
                        type="text"
                        class="rounded-md border bg-background px-3 py-2"
                        bind:value={location}
                        placeholder="e.g. Zoom, Office, Online"
                    />
                </label>

                <Dialog.Footer class="flex justify-end gap-2">
                    <Dialog.Close asChild>
                        <Button type="button" variant="secondary">Cancel</Button
                        >
                    </Dialog.Close>
                    <Button type="submit">Create</Button>
                </Dialog.Footer>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<!-- Table -->
<div class="overflow-x-auto rounded-lg border">
    <table class="min-w-full text-sm">
        <thead class="bg-muted/50 text-left">
            <tr>
                <th class="px-4 py-2">ID</th>
                <th class="px-4 py-2">Type</th>
                <th class="px-4 py-2">Date</th>
                <th class="px-4 py-2">Location</th>
                <th class="px-4 py-2">User</th>
                <th class="px-4 py-2">Person</th>
                <th class="px-4 py-2">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $contacts as c (c.id)}
                <tr class="border-t">
                    <td class="px-4 py-2">{c.id}</td>
                    <td class="px-4 py-2">{c.contactType}</td>
                    <td class="px-4 py-2">{c.contactDate}</td>
                    <td class="px-4 py-2">{c.location}</td>
                    <td class="px-4 py-2">{c.userId}</td>
                    <td class="px-4 py-2">{c.personId}</td>
                    <td class="px-4 py-2">
                        <button
                            class="rounded-md border px-2 py-1 hover:bg-destructive hover:text-destructive-foreground"
                            on:click={() => handleDelete(c.id)}
                        >
                            Delete
                        </button>
                    </td>
                </tr>
            {/each}
            {#if $contacts.length === 0}
                <tr class="border-t">
                    <td
                        colspan="7"
                        class="text-center py-8 text-muted-foreground"
                    >
                        No interactions yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
