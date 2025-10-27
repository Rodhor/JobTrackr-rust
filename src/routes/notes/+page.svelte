<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import type { Note } from "$lib/types/note";
    import type { NoteType } from "$lib/types/enums";
    import { notes } from "$lib/stores/notes";

    // ----------------------------------------------------------
    // Mock data (showcase only)
    // ----------------------------------------------------------
    const mockNotes: Note[] = [
        {
            id: 1,
            noteType: "general",
            content: "Follow up after interview scheduled for next week.",
            applicationId: 101,
            companyId: 2,
            createAt: "2025-10-18 09:30:00",
            updatedAt: "2025-10-19 10:00:00",
        },
        {
            id: 2,
            noteType: "feedback",
            content: "Called HR to confirm receipt of application documents.",
            contactId: 15,
            companyId: 4,
            createAt: "2025-10-12 11:15:00",
            updatedAt: "2025-10-12 11:15:00",
        },
        {
            id: 3,
            noteType: "general",
            content:
                "Research company benefits and prepare negotiation points.",
            companyId: 3,
            createAt: "2025-09-28 14:45:00",
            updatedAt: "2025-09-30 08:10:00",
        },
    ];

    $notes = mockNotes;

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function typeColor(type?: NoteType) {
        switch (type) {
            case "other":
                return "bg-blue-100 text-blue-800";
            case "feedback":
                return "bg-green-100 text-green-800";
            case "general":
                return "bg-yellow-100 text-yellow-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Notes</h1>
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
                <th class="px-4 py-3">Content</th>
                <th class="px-4 py-3">Linked IDs</th>
                <th class="px-4 py-3">Created</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $notes as n (n.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3">
                        <Badge class={typeColor(n.noteType)}>
                            {n.noteType || "—"}
                        </Badge>
                    </td>
                    <td
                        class="px-4 py-3 truncate max-w-[300px] text-muted-foreground"
                    >
                        {n.content || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if n.applicationId}
                            App #{n.applicationId}
                        {:else if n.contactId}
                            Contact #{n.contactId}
                        {:else if n.personId}
                            Person #{n.personId}
                        {:else if n.companyId}
                            Company #{n.companyId}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(n.createAt)}</td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $notes.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="5"
                    >
                        No notes yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
