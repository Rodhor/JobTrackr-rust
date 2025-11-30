<script lang="ts">
    import { applications } from "$lib/stores/applications";
    import { companies } from "$lib/stores/companies";
    import { jobListings } from "$lib/stores/jobListings";
    import { interactions } from "$lib/stores/interactions";
    import { notes } from "$lib/stores/notes";
    import { reminders } from "$lib/stores/reminders";
    import { people } from "$lib/stores/people";
    import { StageDisplay } from "$lib/types/enums";
    import {
        Card,
        CardContent,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card";
    import { Badge } from "$lib/components/ui/badge";
    import { Separator } from "$lib/components/ui/separator";
    import BriefcaseIcon from "lucide-svelte/icons/briefcase";
    import BuildingIcon from "lucide-svelte/icons/building-2";
    import ListIcon from "lucide-svelte/icons/list";
    import UserIcon from "lucide-svelte/icons/user";
    import MessageSquareIcon from "lucide-svelte/icons/message-square";
    import FileTextIcon from "lucide-svelte/icons/file-text";
    import ClockIcon from "lucide-svelte/icons/clock";
    import CheckCircleIcon from "lucide-svelte/icons/check-circle";
    import AlertCircleIcon from "lucide-svelte/icons/alert-circle";

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
    <div>
        <h1 class="text-4xl font-bold tracking-tight">Dashboard</h1>
        <p class="text-muted-foreground mt-2">
            Overview of your job search progress
        </p>
    </div>

    <!-- ----------------------------------------------------------
    Top Summary Cards
    ----------------------------------------------------------- -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card class="hover:shadow-md transition-shadow">
            <CardHeader class="pb-3">
                <div class="flex items-center justify-between">
                    <CardTitle class="text-sm font-medium"
                        >Applications</CardTitle
                    >
                    <BriefcaseIcon
                        class="size-5 text-muted-foreground opacity-60"
                    />
                </div>
            </CardHeader>
            <CardContent>
                <div class="text-3xl font-bold">{totalApplications}</div>
                <p class="text-xs text-muted-foreground mt-2">
                    Total tracked applications
                </p>
            </CardContent>
        </Card>

        <Card class="hover:shadow-md transition-shadow">
            <CardHeader class="pb-3">
                <div class="flex items-center justify-between">
                    <CardTitle class="text-sm font-medium">Companies</CardTitle>
                    <BuildingIcon
                        class="size-5 text-muted-foreground opacity-60"
                    />
                </div>
            </CardHeader>
            <CardContent>
                <div class="text-3xl font-bold">{totalCompanies}</div>
                <p class="text-xs text-muted-foreground mt-2">
                    Active companies in database
                </p>
            </CardContent>
        </Card>

        <Card class="hover:shadow-md transition-shadow">
            <CardHeader class="pb-3">
                <div class="flex items-center justify-between">
                    <CardTitle class="text-sm font-medium"
                        >Job Listings</CardTitle
                    >
                    <ListIcon class="size-5 text-muted-foreground opacity-60" />
                </div>
            </CardHeader>
            <CardContent>
                <div class="text-3xl font-bold">{totalJobListings}</div>
                <p class="text-xs text-muted-foreground mt-2">Saved listings</p>
            </CardContent>
        </Card>

        <Card class="hover:shadow-md transition-shadow">
            <CardHeader class="pb-3">
                <div class="flex items-center justify-between">
                    <CardTitle class="text-sm font-medium">Contacts</CardTitle>
                    <UserIcon class="size-5 text-muted-foreground opacity-60" />
                </div>
            </CardHeader>
            <CardContent>
                <div class="text-3xl font-bold">{totalPeople}</div>
                <p class="text-xs text-muted-foreground mt-2">People tracked</p>
            </CardContent>
        </Card>
    </div>

    <Separator />

    <!-- ----------------------------------------------------------
    Application Stage Distribution
    ----------------------------------------------------------- -->
    <div>
        <h2 class="text-xl font-bold mb-4">Application Stages</h2>
        {#if stageCounts.length > 0}
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
                {#each stageCounts as [stage, count]}
                    <Card class="hover:shadow-md transition-shadow">
                        <CardContent
                            class="py-4 flex items-center justify-between"
                        >
                            <span class="text-sm font-medium">
                                {StageDisplay[
                                    stage as keyof typeof StageDisplay
                                ] ?? stage}
                            </span>
                            <Badge class="ml-2">{count}</Badge>
                        </CardContent>
                    </Card>
                {/each}
            </div>
        {:else}
            <div
                class="rounded-lg border border-dashed border-border bg-muted/30 p-8 text-center"
            >
                <BriefcaseIcon
                    class="mx-auto mb-3 size-8 text-muted-foreground opacity-40"
                />
                <p class="text-sm text-muted-foreground">
                    No applications yet.
                </p>
            </div>
        {/if}
    </div>

    <Separator />

    <!-- ----------------------------------------------------------
    Reminders & Other Stats
    ----------------------------------------------------------- -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card class="hover:shadow-md transition-shadow">
            <CardHeader>
                <div class="flex items-center justify-between">
                    <CardTitle class="flex items-center gap-2">
                        <ClockIcon class="size-5" />
                        Reminders
                    </CardTitle>
                </div>
            </CardHeader>
            <CardContent class="space-y-4">
                <div
                    class="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                >
                    <div class="flex items-center gap-2">
                        <AlertCircleIcon class="size-4 text-orange-600" />
                        <span class="text-sm text-muted-foreground">Active</span
                        >
                    </div>
                    <span class="font-bold text-lg">{activeReminders}</span>
                </div>
                <div
                    class="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                >
                    <div class="flex items-center gap-2">
                        <CheckCircleIcon class="size-4 text-green-600" />
                        <span class="text-sm text-muted-foreground"
                            >Completed</span
                        >
                    </div>
                    <span class="font-bold text-lg">{completedReminders}</span>
                </div>
                <Separator />
                <p class="text-xs text-muted-foreground">
                    <span class="font-semibold">{totalReminders}</span> total reminders
                </p>
            </CardContent>
        </Card>

        <Card class="hover:shadow-md transition-shadow">
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <span>Other Stats</span>
                </CardTitle>
            </CardHeader>
            <CardContent class="space-y-3">
                <div
                    class="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                >
                    <div class="flex items-center gap-2">
                        <MessageSquareIcon class="size-4 text-blue-600" />
                        <span class="text-sm text-muted-foreground"
                            >Interactions</span
                        >
                    </div>
                    <span class="font-bold text-lg">{totalInteractions}</span>
                </div>
                <div
                    class="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                >
                    <div class="flex items-center gap-2">
                        <FileTextIcon class="size-4 text-purple-600" />
                        <span class="text-sm text-muted-foreground">Notes</span>
                    </div>
                    <span class="font-bold text-lg">{totalNotes}</span>
                </div>
            </CardContent>
        </Card>
    </div>
</div>
