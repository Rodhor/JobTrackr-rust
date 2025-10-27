<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { reminders } from "$lib/stores/reminders";
    import type { Reminder } from "$lib/types/reminder";

    const mockReminders: Reminder[] = [
        {
            id: 1,
            noteId: 12,
            title: "Follow-up Email",
            message: "Send follow-up email to recruiter after interview.",
            reminderDate: "2025-10-30",
            isCompleted: false,
            applicationId: 101,
            createAt: "2025-10-20 09:00:00",
            updatedAt: "2025-10-20 09:00:00",
        },
        {
            id: 2,
            noteId: 18,
            title: "Research Company",
            message: "Check company culture before second interview.",
            reminderDate: "2025-10-28",
            isCompleted: true,
            companyId: 3,
            createAt: "2025-10-18 11:30:00",
            updatedAt: "2025-10-22 08:15:00",
        },
        {
            id: 3,
            noteId: 21,
            title: "LinkedIn Message",
            message: "Send thank-you note to hiring manager.",
            reminderDate: "2025-10-27",
            isCompleted: false,
            personId: 7,
            createAt: "2025-10-19 14:00:00",
            updatedAt: "2025-10-19 14:00:00",
        },
    ];

    // Correctly initialize store
    reminders.set(mockReminders);

    function formatDate(dateStr?: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function statusColor(done?: Boolean) {
        return done
            ? "bg-green-100 text-green-800 cursor-pointer"
            : "bg-yellow-100 text-yellow-800 cursor-pointer";
    }

    function toggleStatus(id: number) {
        reminders.update((list) =>
            list.map((r) =>
                r.id === id ? { ...r, isCompleted: !r.isCompleted } : r,
            ),
        );
    }
</script>

<!-- Header -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Reminders</h1>
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
                <th class="px-4 py-3">Message</th>
                <th class="px-4 py-3">Linked IDs</th>
                <th class="px-4 py-3">Reminder Date</th>
                <th class="px-4 py-3">Status</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $reminders as r (r.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        {r.title || "—"}
                    </td>
                    <td
                        class="px-4 py-3 truncate max-w-[300px] text-muted-foreground"
                    >
                        {r.message || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if r.applicationId}
                            App #{r.applicationId}
                        {:else if r.contactId}
                            Contact #{r.contactId}
                        {:else if r.personId}
                            Person #{r.personId}
                        {:else if r.companyId}
                            Company #{r.companyId}
                        {:else if r.jobListingId}
                            Job #{r.jobListingId}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(r.reminderDate)}</td>
                    <td class="px-4 py-3">
                        <button
                            type="button"
                            class="focus:outline-none"
                            on:click={() => toggleStatus(r.id)}
                        >
                            <Badge class={statusColor(r.isCompleted)}>
                                {r.isCompleted ? "Done" : "Pending"}
                            </Badge>
                        </button>
                    </td>
                    <td class="px-4 py-3 text-right">
                        <Button size="sm" variant="destructive">Delete</Button>
                    </td>
                </tr>
            {/each}

            {#if $reminders.length === 0}
                <tr>
                    <td
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                        colspan="6"
                    >
                        No reminders yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
