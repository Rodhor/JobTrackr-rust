<script lang="ts">
    import { contacts } from "$lib/stores/contacts";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { Contact } from "$lib/types/contact";
    import type { ContactType } from "$lib/types/enums";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockContacts: Contact[] = [
        {
            id: 1,
            contactType: "email",
            contactDate: "2025-10-15",
            location: "Remote",
            personId: 12,
            applicationId: 101,
            createdAt: "2025-10-15 09:30:00",
            updatedAt: "2025-10-15 09:30:00",
        },
        {
            id: 2,
            contactType: "phone",
            contactDate: "2025-10-18",
            location: "Office",
            personId: 8,
            applicationId: 104,
            createdAt: "2025-10-18 11:00:00",
            updatedAt: "2025-10-18 11:00:00",
        },
        {
            id: 3,
            contactType: "in_person",
            contactDate: "2025-10-22",
            location: "Berlin HQ",
            personId: 5,
            applicationId: 107,
            createdAt: "2025-10-22 14:00:00",
            updatedAt: "2025-10-22 14:00:00",
        },
    ];

    $contacts = mockContacts;

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function typeColor(type: ContactType) {
        switch (type) {
            case "email":
                return "bg-blue-100 text-blue-800";
            case "phone":
                return "bg-yellow-100 text-yellow-800";
            case "in_person":
                return "bg-green-100 text-green-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Interactions</h1>
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
                <th class="px-4 py-3">Type</th>
                <th class="px-4 py-3">Date</th>
                <th class="px-4 py-3">Location</th>
                <th class="px-4 py-3">Person</th>
                <th class="px-4 py-3">Application</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $contacts as c (c.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3">
                        <Badge class={typeColor(c.contactType)}>
                            {c.contactType}
                        </Badge>
                    </td>
                    <td class="px-4 py-3">{formatDate(c.contactDate)}</td>
                    <td
                        class="px-4 py-3 truncate max-w-[180px] text-muted-foreground"
                    >
                        {c.location || "—"}
                    </td>
                    <td class="px-4 py-3">{c.personId || "—"}</td>
                    <td class="px-4 py-3">{c.applicationId || "—"}</td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $contacts.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="6"
                    >
                        No interactions yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
