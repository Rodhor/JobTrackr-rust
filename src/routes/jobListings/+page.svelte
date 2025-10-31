<script lang="ts">
    import { onMount } from "svelte";
    import {
        jobListings,
        loadJobListings,
        deleteJobListing,
    } from "$lib/stores/jobListings";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import JobListingDialog from "$lib/components/formDialogs/JobListingDialog.svelte";
    import type { JobListing } from "$lib/types/jobListing";
    import { writable } from "svelte/store";
    import {
        WorkType,
        WorkTypeDisplay,
        SeniorityLevelDisplay,
    } from "$lib/types/enums";
    import { companies } from "$lib/stores/companies";

    // ----------------------------------------------------------
    // State
    // ----------------------------------------------------------
    const dialogOpen = writable(false);
    const mode = writable<"create" | "edit">("create");
    const selectedJobListing = writable<JobListing | null>(null);

    // ----------------------------------------------------------
    // Lifecycle
    // ----------------------------------------------------------
    onMount(loadJobListings);

    // ----------------------------------------------------------
    // Handlers
    // ----------------------------------------------------------
    function handleCreate() {
        selectedJobListing.set(null);
        mode.set("create");
        dialogOpen.set(true);
    }

    function handleEdit(listing: JobListing) {
        selectedJobListing.set(listing);
        mode.set("edit");
        dialogOpen.set(true);
    }

    async function handleDelete(id: number) {
        try {
            await deleteJobListing(id);
        } catch (err) {
            console.error("Failed to delete job listing:", err);
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

    function typeColor(type?: WorkType) {
        switch (type) {
            case WorkType.Remote:
                return "bg-blue-100 text-blue-800";
            case WorkType.InOffice:
                return "bg-green-100 text-green-800";
            case WorkType.Hybrid:
                return "bg-yellow-100 text-yellow-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- ----------------------------------------------------------
Header
<!-- ----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Listings</h1>

    {#if $companies.length === 0}
        <Button
            disabled
            variant="secondary"
            title="Add a company before creating applications."
        >
            Add a company first
        </Button>
    {:else}
        <Button onclick={handleCreate}>New Listing</Button>
    {/if}
</div>

<JobListingDialog
    bind:open={$dialogOpen}
    mode={$mode}
    existingJobListing={$selectedJobListing}
/>

<!-- ----------------------------------------------------------
Table
----------------------------------------------------------- -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Title</th>
                <th class="px-4 py-3">Work Type</th>
                <th class="px-4 py-3">Seniority</th>
                <th class="px-4 py-3">Salary</th>
                <th class="px-4 py-3">Created</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>

        <tbody>
            {#each $jobListings as j (j.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground"
                        >{j.title}</td
                    >
                    <td class="px-4 py-3">
                        <Badge class={typeColor(j.workType as WorkType)}>
                            {WorkTypeDisplay[j.workType as WorkType] || "—"}
                        </Badge>
                    </td>
                    <td class="px-4 py-3">
                        {#if j.seniorityLevel}
                            {SeniorityLevelDisplay[j.seniorityLevel]}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if j.salaryMin && j.salaryMax}
                            {j.salaryMin}–{j.salaryMax} {j.currency}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(j.createdAt)}</td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            onclick={() => handleEdit(j)}
                        >
                            Edit
                        </Button>
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => handleDelete(j.id)}
                        >
                            Delete
                        </Button>
                    </td>
                </tr>
            {/each}

            {#if $jobListings.length === 0}
                <tr>
                    <td
                        colspan="6"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No job listings yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
