<script lang="ts">
    import { onMount } from "svelte";
    import { resolve } from "$app/paths";

    import {
        Card,
        CardHeader,
        CardTitle,
        CardContent,
        CardDescription,
    } from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { Briefcase, Bell, StickyNote, ArrowRight } from "lucide-svelte";

    import { applications, loadApplications } from "$lib/stores/applications";
    import { notes, loadNotes } from "$lib/stores/notes";
    import { jobListings, loadJobListings } from "$lib/stores/jobListings";
    import { people, loadPeople } from "$lib/stores/people";
    import { reminders, loadReminders } from "$lib/stores/reminders";
    import { companies, loadCompanies } from "$lib/stores/companies";
    import { interactions, loadInteractions } from "$lib/stores/interactions";

    onMount(async () => {
        await Promise.all([
            loadApplications(),
            loadCompanies(),
            loadNotes(),
            loadJobListings(),
            loadPeople(),
            loadReminders(),
            loadInteractions(),
        ]);
    });

    function formatDate(dateString?: string) {
        if (!dateString) return "-";
        const d = new Date(dateString);
        return isNaN(d.getTime())
            ? "-"
            : d.toLocaleDateString(undefined, {
                  year: "numeric",
                  month: "short",
                  day: "numeric",
              });
    }
</script>

<section class="space-y-10">
    <div>
        <h1 class="text-3xl font-semibold tracking-tight mb-2">Welcome back</h1>
        <p class="text-sm text-muted-foreground mb-6">
            Here’s a quick look at your current job search activity.
        </p>

        <div class="grid gap-6 md:grid-cols-3">
            <!-- =========================================================== -->
            <!-- Applications -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <Briefcase class="h-5 w-5 text-primary" />
                        Applications
                    </CardTitle>
                    <CardDescription>
                        Recent activity and statuses
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if $applications.length === 0}
                        <p class="text-sm text-muted-foreground italic">
                            No applications found.
                        </p>
                    {:else}
                        {#each $applications.slice(0, 5) as app, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <div>
                                    <p class="font-medium leading-tight">
                                        {app.displayLabel}
                                    </p>
                                    <p class="text-sm text-muted-foreground">
                                        {app.stage}
                                    </p>
                                </div>
                                <span class="text-xs text-muted-foreground">
                                    {formatDate(app.appliedDate)}
                                </span>
                            </div>
                            {#if i < Math.min($applications.length, 5) - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4">
                        <a href={resolve("/applications")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- =========================================================== -->
            <!-- Reminders -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <Bell class="h-5 w-5 text-primary" />
                        Reminders
                    </CardTitle>
                    <CardDescription>
                        Pending reminders and due dates
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if $reminders.filter((r) => !r.isCompleted).length === 0}
                        <p class="text-sm text-muted-foreground italic">
                            No pending reminders.
                        </p>
                    {:else}
                        {#each $reminders
                            .filter((r) => !r.isCompleted)
                            .slice(0, 5) as r, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <p class="font-medium truncate">
                                    {r.displayLabel}
                                </p>
                                <span class="text-xs text-muted-foreground">
                                    {formatDate(r.reminderDate)}
                                </span>
                            </div>
                            {#if i < Math.min($reminders.filter((r) => !r.isCompleted).length, 5) - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4">
                        <a href={resolve("/reminders")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- =========================================================== -->
            <!-- Notes -->
            <!-- =========================================================== -->
            <Card class="hover:bg-muted/10 transition-colors">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2">
                        <StickyNote class="h-5 w-5 text-primary" />
                        Notes
                    </CardTitle>
                    <CardDescription>
                        Recent reminders or thoughts
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if $notes.length === 0}
                        <p class="text-sm text-muted-foreground italic">
                            No notes found.
                        </p>
                    {:else}
                        {#each $notes.slice(0, 5) as note, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <p class="font-medium truncate">
                                    {note.displayLabel}
                                </p>
                                <span class="text-xs text-muted-foreground">
                                    {formatDate(note.updatedAt)}
                                </span>
                            </div>
                            {#if i < Math.min($notes.length, 5) - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4">
                        <a href={resolve("/notes")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>
        </div>
    </div>
</section>
