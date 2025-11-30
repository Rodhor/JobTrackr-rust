<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import {
        reminders,
        deleteReminder,
        updateReminder,
    } from "$lib/stores/reminders";
    import type { Reminder } from "$lib/types/reminder";
    import Trash2Icon from "lucide-svelte/icons/trash-2";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import CheckCircleIcon from "lucide-svelte/icons/check-circle";
    import AlertCircleIcon from "lucide-svelte/icons/alert-circle";
    import ClockIcon from "lucide-svelte/icons/clock";

    async function handleDelete(id: number) {
        await deleteReminder(id);
    }

    async function toggleStatus(r: Reminder) {
        if (r.id !== undefined) {
            await updateReminder(r.id, { isCompleted: !r.isCompleted });
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

    function getReminderStatus(reminder: Reminder) {
        if (reminder.isCompleted) {
            return {
                type: "completed",
                label: "Done",
                color: "bg-green-100 text-green-800",
                icon: CheckCircleIcon,
            };
        }

        const reminderDate = new Date(reminder.reminderDate);
        const today = new Date();
        today.setHours(0, 0, 0, 0);
        reminderDate.setHours(0, 0, 0, 0);

        if (reminderDate < today) {
            return {
                type: "overdue",
                label: "Overdue",
                color: "bg-red-100 text-red-800",
                icon: AlertCircleIcon,
            };
        } else if (reminderDate.getTime() === today.getTime()) {
            return {
                type: "today",
                label: "Due Today",
                color: "bg-orange-100 text-orange-800",
                icon: ClockIcon,
            };
        } else {
            return {
                type: "pending",
                label: "Pending",
                color: "bg-blue-100 text-blue-800",
                icon: CalendarIcon,
            };
        }
    }

    function getLinkedInfo(reminder: Reminder) {
        if (reminder.applicationId)
            return {
                label: `Application #${reminder.applicationId}`,
                type: "application",
            };
        if (reminder.companyId)
            return { label: `Company #${reminder.companyId}`, type: "company" };
        if (reminder.personId)
            return { label: `Person #${reminder.personId}`, type: "person" };
        if (reminder.jobListingId)
            return {
                label: `Job Listing #${reminder.jobListingId}`,
                type: "job",
            };
        if (reminder.noteId)
            return { label: `Note #${reminder.noteId}`, type: "note" };
        return { label: "—", type: "none" };
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Reminders</h1>
        <p class="text-muted-foreground mt-1">
            Manage and track your reminders
        </p>
    </div>
    <Button href="/reminders/create" size="lg">+ New Reminder</Button>
</div>

<!-- ----------------------------------------------------------
Stats Cards
----------------------------------------------------------- -->
<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">Total</div>
        <div class="text-2xl font-bold">{$reminders.length}</div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Completed
        </div>
        <div class="text-2xl font-bold text-green-600">
            {$reminders.filter((r) => r.isCompleted).length}
        </div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Due Today
        </div>
        <div class="text-2xl font-bold text-orange-600">
            {$reminders.filter((r) => {
                const status = getReminderStatus(r);
                return status.type === "today";
            }).length}
        </div>
    </div>
    <div class="rounded-lg border border-border bg-background p-4">
        <div class="text-sm font-medium text-muted-foreground mb-1">
            Overdue
        </div>
        <div class="text-2xl font-bold text-red-600">
            {$reminders.filter((r) => {
                const status = getReminderStatus(r);
                return status.type === "overdue";
            }).length}
        </div>
    </div>
</div>

<!-- ----------------------------------------------------------
Table
----------------------------------------------------------- -->
{#if $reminders.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <CalendarIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No reminders yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Create your first reminder to get started
        </p>
        <Button href="/reminders/create">Create Reminder</Button>
    </div>
{:else}
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground border-b border-border"
            >
                <tr>
                    <th class="px-6 py-4 font-semibold">Title</th>
                    <th class="px-6 py-4 font-semibold">Message</th>
                    <th class="px-6 py-4 font-semibold">Linked To</th>
                    <th class="px-6 py-4 font-semibold">Date</th>
                    <th class="px-6 py-4 font-semibold">Status</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each $reminders as r (r.id)}
                    {@const status = getReminderStatus(r)}
                    {@const linkedInfo = getLinkedInfo(r)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Title -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {r.title || "—"}
                            </div>
                        </td>

                        <!-- Message -->
                        <td class="px-6 py-4">
                            <div
                                class="text-muted-foreground truncate max-w-xs"
                            >
                                {r.message || "—"}
                            </div>
                        </td>

                        <!-- Linked To -->
                        <td class="px-6 py-4">
                            <span class="text-muted-foreground text-sm"
                                >{linkedInfo.label}</span
                            >
                        </td>

                        <!-- Date -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {formatDate(r.reminderDate)}
                            </div>
                        </td>

                        <!-- Status Badge -->
                        <td class="px-6 py-4">
                            <button
                                type="button"
                                class="focus:outline-none hover:opacity-80 transition-opacity"
                                onclick={() => toggleStatus(r)}
                                title={r.isCompleted
                                    ? "Mark as pending"
                                    : "Mark as completed"}
                            >
                                <Badge
                                    class={`${status.color} cursor-pointer flex items-center gap-1. 5`}
                                >
                                    <svelte:component
                                        this={status.icon}
                                        class="size-3"
                                    />
                                    {status.label}
                                </Badge>
                            </button>
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/reminders/{r.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    class="gap-2"
                                    onclick={() => r.id && handleDelete(r.id)}
                                >
                                    <Trash2Icon class="size-4" />
                                    Delete
                                </Button>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
