<script lang="ts">
    import { onMount } from "svelte";
    import {
        jobListings,
        loadJobListings,
        createJobListing,
        deleteJobListing,
    } from "$lib/stores/jobListings";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

    let open = false;
    let companyId = 1;
    let title = "";
    let category = "";

    onMount(loadJobListings);

    async function handleCreate() {
        await createJobListing({
            companyId,
            title,
            category,
            workType: undefined,
            seniorityLevel: undefined,
            salaryMin: undefined,
            salaryMax: undefined,
            currency: undefined,
            description: undefined,
            url: undefined,
        });
        open = false;
        title = category = "";
    }

    async function handleDelete(id: number) {
        await deleteJobListing(id);
    }
</script>

<div class="mb-4 flex justify-between">
    <h1 class="text-xl font-semibold">Job Listings</h1>
    <Dialog.Root bind:open>
        <Dialog.Trigger asChild><Button>New Listing</Button></Dialog.Trigger>
        <Dialog.Content>
            <Dialog.Header
                ><Dialog.Title>Create Listing</Dialog.Title></Dialog.Header
            >
            <form on:submit|preventDefault={handleCreate} class="space-y-4">
                <input
                    bind:value={title}
                    placeholder="Title"
                    class="w-full border rounded-md px-3 py-2"
                    required
                />
                <input
                    bind:value={category}
                    placeholder="Category"
                    class="w-full border rounded-md px-3 py-2"
                />
                <Button type="submit">Create</Button>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<table class="min-w-full border text-sm">
    <thead class="bg-muted/50">
        <tr
            ><th class="px-4 py-2">ID</th><th class="px-4 py-2">Title</th><th
                class="px-4 py-2">Company</th
            ><th class="px-4 py-2">Actions</th></tr
        >
    </thead>
    <tbody>
        {#each $jobListings as j (j.id)}
            <tr class="border-t">
                <td class="px-4 py-2">{j.id}</td>
                <td class="px-4 py-2">{j.title}</td>
                <td class="px-4 py-2">{j.companyId}</td>
                <td class="px-4 py-2">
                    <button
                        on:click={() => handleDelete(j.id)}
                        class="border px-2 py-1 rounded-md hover:bg-destructive hover:text-white"
                    >
                        Delete
                    </button>
                </td>
            </tr>
        {/each}
        {#if $jobListings.length === 0}
            <tr
                ><td colspan="4" class="text-center py-4 text-muted-foreground"
                    >No job listings yet.</td
                ></tr
            >
        {/if}
    </tbody>
</table>
