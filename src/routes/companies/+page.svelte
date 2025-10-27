<script lang="ts">
    import { companies } from "$lib/stores/companies";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { Company } from "$lib/types/company";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockCompanies: Company[] = [
        {
            id: 1,
            name: "TechCorp",
            city: "Berlin",
            country: "Germany",
            industry: "Software Development",
            website: "https://techcorp.example.com",
            createdAt: "2025-09-10 08:45:00",
            updatedAt: "2025-09-25 12:20:00",
        },
        {
            id: 2,
            name: "InnovaWorks",
            city: "Copenhagen",
            country: "Denmark",
            industry: "Consulting",
            website: "https://innova.example.com",
            createdAt: "2025-10-02 10:15:00",
            updatedAt: "2025-10-22 11:05:00",
        },
        {
            id: 3,
            name: "DataForge",
            city: "Munich",
            country: "Germany",
            industry: "Data Analytics",
            website: "https://dataforge.example.com",
            createdAt: "2025-09-20 09:10:00",
            updatedAt: "2025-10-10 16:30:00",
        },
        {
            id: 4,
            name: "FutureLab",
            city: "Stockholm",
            country: "Sweden",
            industry: "Research",
            website: "https://futurelab.example.com",
            createdAt: "2025-08-05 13:00:00",
            updatedAt: "2025-09-01 09:45:00",
        },
    ];

    $companies = mockCompanies;

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Companies</h1>
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
                <th class="px-4 py-3">Name</th>
                <th class="px-4 py-3">City</th>
                <th class="px-4 py-3">Country</th>
                <th class="px-4 py-3">Industry</th>
                <th class="px-4 py-3">Last Updated</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $companies as c (c.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        {c.name}
                    </td>
                    <td class="px-4 py-3">{c.city}</td>
                    <td class="px-4 py-3">{c.country}</td>
                    <td
                        class="px-4 py-3 truncate max-w-[180px] text-muted-foreground"
                    >
                        {c.industry || "—"}
                    </td>
                    <td class="px-4 py-3">{formatDate(c.updatedAt)}</td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $companies.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="6"
                    >
                        No companies yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
