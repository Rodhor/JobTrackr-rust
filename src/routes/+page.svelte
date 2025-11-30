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
    import {
        Briefcase,
        Bell,
        StickyNote,
        ArrowRight,
        TrendingUp,
    } from "lucide-svelte";

    // Import all stores
    import { applications, loadApplications } from "$lib/stores/applications";
    import { reminders, loadReminders } from "$lib/stores/reminders";
    import { notes, loadNotes } from "$lib/stores/notes";
    import { loadCompanies } from "$lib/stores/companies";
    import { loadJobListings } from "$lib/stores/jobListings";
    import { loadInteractions } from "$lib/stores/interactions";
    import { loadPeople } from "$lib/stores/people";

    onMount(async () => {
        await Promise.all([loadApplications(), loadReminders(), loadNotes()]);

        Promise.allSettled([
            loadCompanies(),
            loadJobListings(),
            loadInteractions(),
            loadPeople(),
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

    // Derived statistics
    $: pendingReminders = $reminders.filter((r) => !r.isCompleted);
    $: recentApplications = $applications.slice(0, 5);
    $: recentNotes = $notes.slice(0, 5);
    $: offeredApplications = $applications.filter(
        (a) => a.stage === "offered",
    ).length;
</script>

<section class="space-y-10">
    <div>
        <div class="mb-8">
            <h1 class="text-4xl font-bold tracking-tight">Welcome back</h1>
            <p class="text-muted-foreground mt-2">
                Here's a quick look at your current job search activity.
            </p>
        </div>

        <!-- ----------------------------------------------------------
        Quick Stats
        ----------------------------------------------------------- -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
            <div class="rounded-lg border border-border bg-background p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div
                            class="text-sm font-medium text-muted-foreground mb-1"
                        >
                            Total Applications
                        </div>
                        <div class="text-3xl font-bold">
                            {$applications.length}
                        </div>
                    </div>
                    <TrendingUp
                        class="size-10 text-muted-foreground opacity-20"
                    />
                </div>
            </div>
            <div class="rounded-lg border border-border bg-background p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div
                            class="text-sm font-medium text-muted-foreground mb-1"
                        >
                            Pending Reminders
                        </div>
                        <div class="text-3xl font-bold text-orange-600">
                            {pendingReminders.length}
                        </div>
                    </div>
                    <Bell class="size-10 text-muted-foreground opacity-20" />
                </div>
            </div>
            <div class="rounded-lg border border-border bg-background p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div
                            class="text-sm font-medium text-muted-foreground mb-1"
                        >
                            Offers
                        </div>
                        <div class="text-3xl font-bold text-green-600">
                            {offeredApplications}
                        </div>
                    </div>
                    <Briefcase
                        class="size-10 text-muted-foreground opacity-20"
                    />
                </div>
            </div>
        </div>

        <!-- ----------------------------------------------------------
        Activity Cards
        ----------------------------------------------------------- -->
        <div class="grid gap-6 md:grid-cols-3">
            <!-- ======================================================= -->
            <!-- Applications -->
            <!-- ======================================================= -->
            <Card class="hover:shadow-md transition-shadow">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2 text-lg">
                        <Briefcase class="h-5 w-5 text-blue-600" />
                        Applications
                    </CardTitle>
                    <CardDescription>
                        Recent activity and statuses
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if recentApplications.length === 0}
                        <div
                            class="rounded-lg border border-dashed border-border bg-muted/30 p-6 text-center"
                        >
                            <p class="text-sm text-muted-foreground">
                                No applications yet
                            </p>
                        </div>
                    {:else}
                        {#each recentApplications as app, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <div class="flex-1 min-w-0">
                                    <p
                                        class="font-medium leading-tight truncate"
                                    >
                                        {app.displayLabel}
                                    </p>
                                    <p class="text-xs text-muted-foreground">
                                        {app.stage}
                                    </p>
                                </div>
                                <span
                                    class="text-xs text-muted-foreground ml-2 flex-shrink-0"
                                >
                                    {formatDate(app.appliedDate)}
                                </span>
                            </div>
                            {#if i < recentApplications.length - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4 pt-2 border-t">
                        <a href={resolve("/applications")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1 text-xs"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- ======================================================= -->
            <!-- Reminders -->
            <!-- ======================================================= -->
            <Card class="hover:shadow-md transition-shadow">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2 text-lg">
                        <Bell class="h-5 w-5 text-orange-600" />
                        Reminders
                    </CardTitle>
                    <CardDescription>
                        Pending reminders and due dates
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if pendingReminders.length === 0}
                        <div
                            class="rounded-lg border border-dashed border-border bg-muted/30 p-6 text-center"
                        >
                            <p class="text-sm text-muted-foreground">
                                No pending reminders
                            </p>
                        </div>
                    {:else}
                        {#each pendingReminders.slice(0, 5) as r, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <div class="flex-1 min-w-0">
                                    <p class="font-medium truncate">
                                        {r.title}
                                    </p>
                                    <p class="text-xs text-muted-foreground">
                                        {r.message?.substring(0, 40) ??
                                            "No message"}
                                        {(r.message?.length ?? 0) > 40
                                            ? "..."
                                            : ""}
                                    </p>
                                </div>
                                <span
                                    class="text-xs text-muted-foreground ml-2 flex-shrink-0"
                                >
                                    {formatDate(r.reminderDate)}
                                </span>
                            </div>
                            {#if i < Math.min(pendingReminders.length, 5) - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4 pt-2 border-t">
                        <a href={resolve("/reminders")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1 text-xs"
                            >
                                View All <ArrowRight class="h-3 w-3" />
                            </Button>
                        </a>
                    </div>
                </CardContent>
            </Card>

            <!-- ======================================================= -->
            <!-- Notes -->
            <!-- ======================================================= -->
            <Card class="hover:shadow-md transition-shadow">
                <CardHeader class="pb-3">
                    <CardTitle class="flex items-center gap-2 text-lg">
                        <StickyNote class="h-5 w-5 text-purple-600" />
                        Notes
                    </CardTitle>
                    <CardDescription>
                        Recent reminders or thoughts
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-3">
                    {#if recentNotes.length === 0}
                        <div
                            class="rounded-lg border border-dashed border-border bg-muted/30 p-6 text-center"
                        >
                            <p class="text-sm text-muted-foreground">
                                No notes yet
                            </p>
                        </div>
                    {:else}
                        {#each recentNotes as note, i}
                            <div
                                class="flex justify-between items-center rounded-md px-3 py-2 hover:bg-accent/20 transition-colors"
                            >
                                <div class="flex-1 min-w-0">
                                    <p class="font-medium truncate">
                                        {note.title}
                                    </p>
                                    <p class="text-xs text-muted-foreground">
                                        {note.content?.substring(0, 40) ??
                                            "No content"}
                                        {(note.content?.length ?? 0) > 40
                                            ? "..."
                                            : ""}
                                    </p>
                                </div>
                                <span
                                    class="text-xs text-muted-foreground ml-2 flex-shrink-0"
                                >
                                    {formatDate(note.updatedAt)}
                                </span>
                            </div>
                            {#if i < recentNotes.length - 1}
                                <Separator />
                            {/if}
                        {/each}
                    {/if}

                    <div class="flex justify-end mt-4 pt-2 border-t">
                        <a href={resolve("/notes")}>
                            <Button
                                variant="ghost"
                                size="sm"
                                class="flex items-center gap-1 text-xs"
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
