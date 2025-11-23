<script lang="ts">
    import { onMount } from "svelte";
    import {
        companies,
        loadCompanies,
        deleteCompany,
    } from "$lib/stores/companies";
    import { Button } from "$lib/components/ui/button";

    // ----------------------------------------------------------
    // Lifecycle
    // ----------------------------------------------------------
    onMount(loadCompanies);

    async function handleDelete(id: number) {
        try {
            await deleteCompany(id);
        } catch (err) {
            console.error("Failed to delete company:", err);
        }
    }

    function formatDate(dateStr: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Companies</h1>
    <Button href="/companies/create">New Company</Button>
</div>

<!-- ----------------------------------------------------------
Companies Table
----------------------------------------------------------- -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Name</th>
                <th class="px-4 py-3">City</th>
                <th class="px-4 py-3">Country</th>
                <th class="px-4 py-3">Industry</th>
                <th class="px-4 py-3">Website</th>
                <th class="px-4 py-3">Last Updated</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $companies as c (c.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground"
                        >{c.name}</td
                    >
                    <td class="px-4 py-3">{c.city}</td>
                    <td class="px-4 py-3">{c.country}</td>
                    <td
                        class="px-4 py-3 text-muted-foreground truncate max-w-[160px]"
                    >
                        {c.industry || "—"}
                    </td>
                    <td
                        class="px-4 py-3 text-blue-600 hover:underline truncate max-w-[200px]"
                    >
                        <a href={c.website} target="_blank">{c.website}</a>
                    </td>
                    <td class="px-4 py-3">
                        {c.updatedAt ? formatDate(c.updatedAt) : "—"}
                    </td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            href="/companies/{c.id}">Edit</Button
                        >
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => c.id && handleDelete(c.id)}
                        >
                            Delete
                        </Button>
                    </td>
                </tr>
            {/each}

            {#if $companies.length === 0}
                <tr>
                    <td
                        colspan="7"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No companies yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
