<script lang="ts">
    import { jobListings, deleteJobListing } from "$lib/stores/jobListings";
    import { companies } from "$lib/stores/companies";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import {
        WorkType,
        WorkTypeDisplay,
        SeniorityLevelDisplay,
    } from "$lib/types/enums";

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

    function getCompanyName(companyId?: number) {
        if (!companyId) return "—";
        return $companies.find((c) => c.id === companyId)?.name ?? "—";
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
    <Button href="jobListings/create">New Listing</Button>
</div>

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
                <th class="px-4 py-3">Company</th>
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
                    <td class="px-4 py-3 text-muted-foreground">
                        {getCompanyName(j.companyId)}
                    </td>
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
                    <td class="px-4 py-3">
                        {j.createdAt ? formatDate(j.createdAt) : "—"}
                    </td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            href="/jobListings/{j.id}"
                        >
                            Edit
                        </Button>
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => j.id && handleDelete(j.id)}
                        >
                            Delete
                        </Button>
                    </td>
                </tr>
            {/each}

            {#if $jobListings.length === 0}
                <tr>
                    <td
                        colspan="7"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No job listings yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
