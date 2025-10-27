<script lang="ts">
    import { jobListings } from "$lib/stores/jobListings";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { JobListing } from "$lib/types/jobListing";
    import type {
        DefaultWorkType,
        SeniorityLevel,
        Currency,
    } from "$lib/types/enums";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockJobListings: JobListing[] = [
        {
            id: 1,
            companyId: 1,
            title: "Frontend Developer",
            workType: "remote",
            category: "Engineering",
            seniorityLevel: "mid",
            salaryMin: 50000,
            salaryMax: 65000,
            currency: "EUR",
            description: "Develop and maintain UI components using Svelte.",
            url: "https://example.com/frontend-job",
            createdAt: "2025-10-01 09:00:00",
            updatedAt: "2025-10-15 12:00:00",
        },
        {
            id: 2,
            companyId: 2,
            title: "Backend Developer",
            workType: "hybrid",
            category: "Software Engineering",
            seniorityLevel: "senior",
            salaryMin: 70000,
            salaryMax: 85000,
            currency: "EUR",
            description:
                "Implement APIs and database logic using Rust and SQLite.",
            url: "https://example.com/backend-job",
            createdAt: "2025-09-12 11:00:00",
            updatedAt: "2025-09-30 10:30:00",
        },
        {
            id: 3,
            companyId: 3,
            title: "Data Analyst",
            workType: "onsite",
            category: "Analytics",
            seniorityLevel: "junior",
            salaryMin: 40000,
            salaryMax: 48000,
            currency: "EUR",
            description: "Analyze recruitment data and create insight reports.",
            url: "https://example.com/data-analyst",
            createdAt: "2025-08-05 08:45:00",
            updatedAt: "2025-08-20 09:20:00",
        },
    ];

    $jobListings = mockJobListings;

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function typeColor(type?: DefaultWorkType) {
        switch (type) {
            case "remote":
                return "bg-blue-100 text-blue-800";
            case "onsite":
                return "bg-green-100 text-green-800";
            case "hybrid":
                return "bg-yellow-100 text-yellow-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Job Listings</h1>
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
                    <td class="px-4 py-3 font-medium text-foreground">
                        {j.title}
                    </td>
                    <td class="px-4 py-3">
                        <Badge class={typeColor(j.workType)}>
                            {j.workType || "—"}
                        </Badge>
                    </td>
                    <td class="px-4 py-3 capitalize">
                        {j.seniorityLevel || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if j.salaryMin && j.salaryMax}
                            {j.salaryMin}–{j.salaryMax} {j.currency}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(j.createdAt)}</td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $jobListings.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="6"
                    >
                        No job listings yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
