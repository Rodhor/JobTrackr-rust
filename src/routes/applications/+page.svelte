<script lang="ts">
    import { applications } from "$lib/stores/applications";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { Application } from "$lib/types/application";
    import type { Stage } from "$lib/types/enums";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockApplications: Application[] = [
        {
            id: 1,
            jobListingId: 101,
            stage: "applied",
            appliedDate: "2025-10-20",
            cvFilePath: "/home/user/Documents/CV.pdf",
            coverLetterFilePath: "/home/user/Documents/CL.pdf",
            applicationNotes: "Initial application via LinkedIn.",
            createdAt: "2025-10-20 10:34:00",
            updatedAt: "2025-10-20 10:34:00",
        },
        {
            id: 2,
            jobListingId: 102,
            stage: "interviewing",
            appliedDate: "2025-10-15",
            cvFilePath: "/home/user/Documents/CV.pdf",
            coverLetterFilePath: "",
            applicationNotes: "First interview scheduled for 2025-10-25.",
            createdAt: "2025-10-15 14:12:00",
            updatedAt: "2025-10-22 09:10:00",
        },
        {
            id: 3,
            jobListingId: 103,
            stage: "offered",
            appliedDate: "2025-09-30",
            cvFilePath: "/home/user/Documents/CV.pdf",
            coverLetterFilePath: "",
            applicationNotes: "Offer received, pending response.",
            createdAt: "2025-09-30 12:20:00",
            updatedAt: "2025-10-23 18:05:00",
        },
        {
            id: 4,
            jobListingId: 104,
            stage: "rejected",
            appliedDate: "2025-08-12",
            cvFilePath: "",
            coverLetterFilePath: "",
            applicationNotes: "Rejected after second interview.",
            createdAt: "2025-08-12 09:45:00",
            updatedAt: "2025-09-01 11:00:00",
        },
    ];

    $applications = mockApplications;

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function stageColor(stage: Stage) {
        switch (stage) {
            case "applied":
                return "bg-blue-100 text-blue-800";
            case "interviewing":
                return "bg-yellow-100 text-yellow-800";
            case "offered":
                return "bg-green-100 text-green-800";
            case "rejected":
                return "bg-red-100 text-red-800";
            case "withdrawn":
                return "bg-gray-200 text-gray-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Applications</h1>
</div>

<!-- Table -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Job Listing</th>
                <th class="px-4 py-3">Status</th>
                <th class="px-4 py-3">Applied</th>
                <th class="px-4 py-3">Notes</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $applications as a (a.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        #{a.jobListingId}
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
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $applications.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="5"
                    >
                        No applications yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
