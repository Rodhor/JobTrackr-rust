<script lang="ts">
    import { people } from "$lib/stores/people";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { Person } from "$lib/types/person";
    import type { Role } from "$lib/types/enums";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockPeople: Person[] = [
        {
            id: 1,
            firstName: "Alice",
            lastName: "Meyer",
            email: "alice.meyer@example.com",
            phoneNumber: "+49 123 456 789",
            role: "recruiter",
            linkedinUrl: "https://linkedin.com/in/alicemeyer",
            companyId: 1,
            createdAt: "2025-09-12 09:30:00",
            updatedAt: "2025-10-05 12:10:00",
        },
        {
            id: 2,
            firstName: "Jonas",
            lastName: "Keller",
            email: "jonas.keller@example.com",
            phoneNumber: "+45 98 765 432",
            role: "hr",
            linkedinUrl: "https://linkedin.com/in/jonaskeller",
            companyId: 2,
            createdAt: "2025-10-02 11:00:00",
            updatedAt: "2025-10-15 09:45:00",
        },
        {
            id: 3,
            firstName: "Mira",
            lastName: "Lund",
            email: "mira.lund@example.com",
            role: "other",
            companyId: 3,
            createdAt: "2025-09-25 14:00:00",
            updatedAt: "2025-09-25 14:00:00",
        },
    ];

    $people = mockPeople;

    function roleColor(role?: Role) {
        switch (role) {
            case "recruiter":
                return "bg-blue-100 text-blue-800";
            case "hr":
                return "bg-green-100 text-green-800";
            case "other":
                return "bg-yellow-100 text-yellow-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">People</h1>
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
                <th class="px-4 py-3">Role</th>
                <th class="px-4 py-3">Email</th>
                <th class="px-4 py-3">Phone</th>
                <th class="px-4 py-3">LinkedIn</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $people as p (p.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        {p.firstName}
                        {p.lastName}
                    </td>
                    <td class="px-4 py-3">
                        <Badge class={roleColor(p.role)}>
                            {p.role || "—"}
                        </Badge>
                    </td>
                    <td
                        class="px-4 py-3 truncate max-w-[200px] text-muted-foreground"
                    >
                        {p.email || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {p.phoneNumber || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if p.linkedinUrl}
                            <a
                                href={p.linkedinUrl}
                                target="_blank"
                                rel="noreferrer"
                                class="text-blue-600 hover:underline">Profile</a
                            >
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $people.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="6"
                    >
                        No people yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
