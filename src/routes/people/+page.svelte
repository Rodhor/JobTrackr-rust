<script lang="ts">
    import { onMount } from "svelte";
    import {
        people,
        loadPeople,
        createPerson,
        deletePerson,
    } from "$lib/stores/people";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

    let open = false;
    let firstName = "";
    let lastName = "";

    onMount(loadPeople);

    async function handleCreate() {
        await createPerson({
            firstName,
            lastName,
            email: undefined,
            phoneNumber: undefined,
            role: undefined,
            linkedinUrl: undefined,
        });
        open = false;
        firstName = lastName = "";
    }

    async function handleDelete(id: number) {
        await deletePerson(id);
    }
</script>

<div class="mb-4 flex justify-between">
    <h1 class="text-xl font-semibold">People</h1>
    <Dialog.Root bind:open>
        <Dialog.Trigger asChild><Button>New Person</Button></Dialog.Trigger>
        <Dialog.Content>
            <Dialog.Header
                ><Dialog.Title>Create Person</Dialog.Title></Dialog.Header
            >
            <form on:submit|preventDefault={handleCreate} class="space-y-4">
                <input
                    bind:value={firstName}
                    placeholder="First Name"
                    class="w-full border rounded-md px-3 py-2"
                    required
                />
                <input
                    bind:value={lastName}
                    placeholder="Last Name"
                    class="w-full border rounded-md px-3 py-2"
                    required
                />
                <Button type="submit">Create</Button>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<table class="min-w-full border text-sm">
    <thead class="bg-muted/50">
        <tr
            ><th class="px-4 py-2">ID</th><th class="px-4 py-2">Name</th><th
                class="px-4 py-2">Actions</th
            ></tr
        >
    </thead>
    <tbody>
        {#each $people as p (p.id)}
            <tr class="border-t">
                <td class="px-4 py-2">{p.id}</td>
                <td class="px-4 py-2">{p.firstName} {p.lastName}</td>
                <td class="px-4 py-2">
                    <button
                        on:click={() => handleDelete(p.id)}
                        class="border px-2 py-1 rounded-md hover:bg-destructive hover:text-white"
                    >
                        Delete
                    </button>
                </td>
            </tr>
        {/each}
        {#if $people.length === 0}
            <tr
                ><td colspan="3" class="text-center py-4 text-muted-foreground"
                    >No people yet.</td
                ></tr
            >
        {/if}
    </tbody>
</table>
