<script lang="ts">
    import { onMount } from "svelte";
    import { applications, loadApplications } from "$lib/stores/applications";
    import { companies, loadCompanies } from "$lib/stores/companies";
    import { jobListings, loadJobListings } from "$lib/stores/jobListings";
    import { interactions, loadInteractions } from "$lib/stores/interactions";
    import { notes, loadNotes } from "$lib/stores/notes";
    import { reminders, loadReminders } from "$lib/stores/reminders";
    import { people, loadPeople } from "$lib/stores/people";
    import { StageDisplay } from "$lib/types/enums";
    import {
        Card,
        CardContent,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card";
    import { Badge } from "$lib/components/ui/badge";
    import { Separator } from "$lib/components/ui/separator";

    // ----------------------------------------------------------
    // Lifecycle
    // ----------------------------------------------------------
    onMount(async () => {
        await Promise.all([
            loadApplications(),
            loadCompanies(),
            loadJobListings(),
            loadInteractions(),
            loadNotes(),
            loadReminders(),
            loadPeople(),
        ]);
    });

    // ----------------------------------------------------------
    // Derived statistics
    // ----------------------------------------------------------
    $: totalApplications = $applications.length;
    $: totalCompanies = $companies.length;
    $: totalJobListings = $jobListings.length;
    $: totalPeople = $people.length;
    $: totalInteractions = $interactions.length;
    $: totalNotes = $notes.length;
    $: totalReminders = $reminders.length;

    $: activeReminders = $reminders.filter((r) => !r.isCompleted).length;
    $: completedReminders = $reminders.filter((r) => r.isCompleted).length;

    $: stageCounts = Object.entries(
        $applications.reduce(
            (acc, app) => {
                acc[app.stage] = (acc[app.stage] || 0) + 1;
                return acc;
            },
            {} as Record<string, number>,
        ),
    ).sort((a, b) => a[0].localeCompare(b[0]));
</script>

<!-- ----------------------------------------------------------
Dashboard Layout
----------------------------------------------------------- -->
<div class="space-y-8">
    <h1 class="text-3xl font-semibold tracking-tight">Dashboard</h1>

    <!-- Top Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4">
        <Card>
            <CardHeader>
                <CardTitle>Applications</CardTitle>
            </CardHeader>
            <CardContent class="text-3xl font-bold">
                {totalApplications}
                <p class="text-xs text-muted-foreground mt-1">
                    Total tracked applications
                </p>
            </CardContent>
        </Card>

        <Card>
            <CardHeader>
                <CardTitle>Companies</CardTitle>
            </CardHeader>
            <CardContent class="text-3xl font-bold">
                {totalCompanies}
                <p class="text-xs text-muted-foreground mt-1">
                    Active companies in database
                </p>
            </CardContent>
        </Card>

        <Card>
            <CardHeader>
                <CardTitle>Job Listings</CardTitle>
            </CardHeader>
            <CardContent class="text-3xl font-bold">
                {totalJobListings}
                <p class="text-xs text-muted-foreground mt-1">Saved listings</p>
            </CardContent>
        </Card>

        <Card>
            <CardHeader>
                <CardTitle>Contacts</CardTitle>
            </CardHeader>
            <CardContent class="text-3xl font-bold">
                {totalPeople}
                <p class="text-xs text-muted-foreground mt-1">People tracked</p>
            </CardContent>
        </Card>
    </div>

    <Separator />

    <!-- Application Stage Distribution -->
    <div>
        <h2 class="text-lg font-semibold mb-3">Application Stages</h2>
        {#if stageCounts.length > 0}
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
                {#each stageCounts as [stage, count]}
                    <Card>
                        <CardContent
                            class="py-4 flex items-center justify-between"
                        >
                            <span class="text-sm font-medium">
                                {StageDisplay[
                                    stage as keyof typeof StageDisplay
                                ] ?? stage}
                            </span>
                            <Badge>{count}</Badge>
                        </CardContent>
                    </Card>
                {/each}
            </div>
        {:else}
            <p class="text-sm text-muted-foreground">No applications yet.</p>
        {/if}
    </div>

    <Separator />

    <!-- Reminders Summary -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card>
            <CardHeader>
                <CardTitle>Reminders</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between text-sm mb-2">
                    <span class="text-muted-foreground">Active</span>
                    <span class="font-semibold">{activeReminders}</span>
                </div>
                <div class="flex items-center justify-between text-sm">
                    <span class="text-muted-foreground">Completed</span>
                    <span class="font-semibold">{completedReminders}</span>
                </div>
                <Separator class="my-3" />
                <p class="text-xs text-muted-foreground">
                    Total reminders: {totalReminders}
                </p>
            </CardContent>
        </Card>

        <Card>
            <CardHeader>
                <CardTitle>Other Stats</CardTitle>
            </CardHeader>
            <CardContent class="text-sm space-y-2">
                <div class="flex items-center justify-between">
                    <span>Interactions</span>
                    <span class="font-semibold">{totalInteractions}</span>
                </div>
                <div class="flex items-center justify-between">
                    <span>Notes</span>
                    <span class="font-semibold">{totalNotes}</span>
                </div>
            </CardContent>
        </Card>
    </div>
</div>
