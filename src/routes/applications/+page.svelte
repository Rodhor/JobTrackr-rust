<script lang="ts">
    import { onMount } from "svelte";
    import {
        applications,
        loadApplications,
        deleteApplication,
    } from "$lib/stores/applications";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import ApplicationDialog from "$lib/components/formDialogs/ApplicationDialog.svelte";
    import { Stage } from "$lib/types/enums";
    import type { Application } from "$lib/types/application";
    import { writable } from "svelte/store";

    // ----------------------------------------------------------
    // State
    // ----------------------------------------------------------
    const dialogOpen = writable(false);
    const mode = writable<"create" | "edit">("create");
    const selectedApplication = writable<Application | null>(null);

    // ----------------------------------------------------------
    // Lifecycle
    // ----------------------------------------------------------
    onMount(loadApplications);

    // ----------------------------------------------------------
    // Handlers
    // ----------------------------------------------------------
    function handleCreate() {
        selectedApplication.set(null);
        mode.set("create");
        dialogOpen.set(true);
    }

    function handleEdit(application: Application) {
        selectedApplication.set(application);
        mode.set("edit");
        dialogOpen.set(true);
    }

    async function handleDelete(id: number) {
        try {
            await deleteApplication(id);
        } catch (err) {
            console.error("Failed to delete application:", err);
        }
    }

    // ----------------------------------------------------------
    // Helpers
    // ----------------------------------------------------------
    function stageColor(stage: Stage): string {
        switch (stage) {
            case Stage.Applied:
                return "bg-blue-100 text-blue-800";
            case Stage.Screening:
                return "bg-purple-100 text-purple-800";
            case Stage.Assessment:
                return "bg-teal-100 text-teal-800";
            case Stage.Interviewing:
                return "bg-yellow-100 text-yellow-800";
            case Stage.Offered:
                return "bg-green-100 text-green-800";
            case Stage.Rejected:
                return "bg-red-100 text-red-800";
            case Stage.Withdrawn:
                return "bg-gray-200 text-gray-800";
            case Stage.OnHold:
                return "bg-orange-100 text-orange-800";
            default:
                return "bg-muted text-foreground";
        }
    }

    function formatDate(dateStr: string): string {
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
    <h1 class="text-2xl font-semibold tracking-tight">Applications</h1>
    <Button onclick={handleCreate}>New Application</Button>
</div>

<!-- Shared Dialog (used for both create & edit) -->
<ApplicationDialog
    bind:open={$dialogOpen}
    mode={$mode}
    existingApplication={$selectedApplication}
/>

<!-- ----------------------------------------------------------
Applications Table
----------------------------------------------------------- -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Job Listing</th>
                <th class="px-4 py-3">Stage</th>
                <th class="px-4 py-3">Applied</th>
                <th class="px-4 py-3">Notes</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>

        <tbody>
            {#each $applications as a (a.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        {a.displayLabel ?? a.jobListingId ?? "—"}
                    </td>
                    <td class="px-4 py-3">
                        <Badge class={stageColor(a.stage)}>{a.stage}</Badge>
                    </td>
                    <td class="px-4 py-3">{formatDate(a.appliedDate)}</td>
                    <td
                        class="px-4 py-3 truncate max-w-[220px] text-muted-foreground"
                    >
                        {a.applicationNotes || "—"}
                    </td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            onclick={() => handleEdit(a)}
                        >
                            Edit
                        </Button>
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => handleDelete(a.id)}
                        >
                            Delete
                        </Button>
                    </td>
                </tr>
            {/each}

            {#if $applications.length === 0}
                <tr>
                    <td
                        colspan="5"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No applications yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
