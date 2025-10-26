<script lang="ts">
    import { onMount } from "svelte";
    import {
        companies,
        loadCompanies,
        createCompany,
        deleteCompany,
    } from "$lib/stores/companies";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import type { Company } from "$lib/types/company";

    let open = false;

    let name = "";
    let city = "";
    let country = "";
    let industry = "";

    onMount(loadCompanies);

    async function handleCreate() {
        await createCompany({
            name,
            city,
            country,
            industry,
            streetAddress: undefined,
            zipCode: undefined,
            defaultWorkType: undefined,
            website: undefined,
            phoneNumber: undefined,
        });
        open = false;
        name = city = country = industry = "";
    }

    async function handleDelete(id: number) {
        await deleteCompany(id);
    }
</script>

<div class="mb-4 flex justify-between">
    <h1 class="text-xl font-semibold">Companies</h1>
    <Dialog.Root bind:open>
        <Dialog.Trigger asChild>
            <Button>New Company</Button>
        </Dialog.Trigger>

        <Dialog.Content>
            <Dialog.Header>
                <Dialog.Title>Create Company</Dialog.Title>
            </Dialog.Header>
            <form on:submit|preventDefault={handleCreate} class="space-y-4">
                <input
                    bind:value={name}
                    placeholder="Name"
                    class="w-full rounded-md border px-3 py-2"
                    required
                />
                <input
                    bind:value={city}
                    placeholder="City"
                    class="w-full rounded-md border px-3 py-2"
                />
                <input
                    bind:value={country}
                    placeholder="Country"
                    class="w-full rounded-md border px-3 py-2"
                />
                <input
                    bind:value={industry}
                    placeholder="Industry"
                    class="w-full rounded-md border px-3 py-2"
                />
                <Button type="submit">Create</Button>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<table class="min-w-full border text-sm">
    <thead class="bg-muted/50">
        <tr>
            <th class="px-4 py-2">ID</th>
            <th class="px-4 py-2">Name</th>
            <th class="px-4 py-2">City</th>
            <th class="px-4 py-2">Country</th>
            <th class="px-4 py-2">Actions</th>
        </tr>
    </thead>
    <tbody>
        {#each $companies as c (c.id)}
            <tr class="border-t">
                <td class="px-4 py-2">{c.id}</td>
                <td class="px-4 py-2">{c.name}</td>
                <td class="px-4 py-2">{c.city}</td>
                <td class="px-4 py-2">{c.country}</td>
                <td class="px-4 py-2">
                    <button
                        on:click={() => handleDelete(c.id)}
                        class="border px-2 py-1 rounded-md hover:bg-destructive hover:text-white"
                    >
                        Delete
                    </button>
                </td>
            </tr>
        {/each}
        {#if $companies.length === 0}
            <tr
                ><td colspan="5" class="text-center py-4 text-muted-foreground"
                    >No companies yet.</td
                ></tr
            >
        {/if}
    </tbody>
</table>
