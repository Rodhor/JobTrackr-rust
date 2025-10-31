<script lang="ts">
    import { onMount } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import ReminderDialog from "$lib/components/formDialogs/ReminderDialog.svelte";
    import {
        reminders,
        loadReminders,
        deleteReminder,
        updateReminder,
    } from "$lib/stores/reminders";
    import { writable } from "svelte/store";
    import type { Reminder } from "$lib/types/reminder";

    const dialogOpen = writable(false);
    const mode = writable<"create" | "edit">("create");
    const selectedReminder = writable<Reminder | null>(null);

    onMount(loadReminders);

    function handleCreate() {
        selectedReminder.set(null);
        mode.set("create");
        dialogOpen.set(true);
    }

    function handleEdit(reminder: Reminder) {
        selectedReminder.set(reminder);
        mode.set("edit");
        dialogOpen.set(true);
    }

    async function handleDelete(id: number) {
        try {
            await deleteReminder(id);
        } catch (err) {
            console.error("Failed to delete reminder:", err);
        }
    }

    async function toggleStatus(r: Reminder) {
        try {
            await updateReminder(r.id, { isCompleted: !r.isCompleted });
        } catch (err) {
            console.error("Failed to toggle status:", err);
        }
    }

    function formatDate(dateStr?: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function statusColor(done?: boolean) {
        return done
            ? "bg-green-100 text-green-800"
            : "bg-yellow-100 text-yellow-800";
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Reminders</h1>
    <Button onclick={handleCreate}>New Reminder</Button>
</div>

<ReminderDialog
    bind:open={$dialogOpen}
    mode={$mode}
    existingReminder={$selectedReminder}
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
                <th class="px-4 py-3">Message</th>
                <th class="px-4 py-3">Linked</th>
                <th class="px-4 py-3">Date</th>
                <th class="px-4 py-3">Status</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $reminders as r (r.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground"
                        >{r.title || "—"}</td
                    >
                    <td
                        class="px-4 py-3 truncate max-w-[300px] text-muted-foreground"
                        >{r.message || "—"}</td
                    >
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if r.applicationId}
                            App #{r.applicationId}
                        {:else if r.companyId}
                            Company #{r.companyId}
                        {:else if r.personId}
                            Person #{r.personId}
                        {:else if r.jobListingId}
                            Job #{r.jobListingId}
                        {:else if r.noteId}
                            Note #{r.noteId}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(r.reminderDate)}</td>
                    <td class="px-4 py-3">
                        <button
                            type="button"
                            class="focus:outline-none"
                            onclick={() => toggleStatus(r)}
                        >
                            <Badge class={statusColor(r.isCompleted)}>
                                {r.isCompleted ? "Done" : "Pending"}
                            </Badge>
                        </button>
                    </td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            onclick={() => handleEdit(r)}>Edit</Button
                        >
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => handleDelete(r.id)}>Delete</Button
                        >
                    </td>
                </tr>
            {/each}

            {#if $reminders.length === 0}
                <tr>
                    <td
                        colspan="6"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No reminders yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
